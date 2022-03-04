use crate::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use rand::prelude::*;

const COOLDOWN: f32 = 0.85;
pub const ATTACK_PREP_TIME: f32 = 0.55;
const ATTACK_STAB_TIME: f32 = 0.1;
const PARRY_TIME: f32 = 0.5;

pub struct Duel {
    time: f32,
    my_coins: u32,
    your_coins: u32,
    coin_penalty: u32,
    player_cooldown_percent: f32,
}

#[derive(Default, Component)]
pub struct Duelist {
    player: bool,
    x: f32,
    wants_to_attack: bool,
    wants_to_defend: bool,
    direction: f32,
    attacking: bool,
    defending: bool,
    attacked: bool,
    attack_time: f32,
    defend_time: f32,
    cooldown: f32,
    hit: bool,
}

#[derive(Component)]
pub struct DuelHud;

pub struct DuelPlugin;

impl Plugin for DuelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Duel {
            time: 0.,
            my_coins: 0,
            your_coins: 0,
            coin_penalty: 1,
            player_cooldown_percent: 1.,
        })
        .add_system_set(SystemSet::on_enter(MiniGameState::Duel).with_system(init))
        .add_system_set(SystemSet::on_update(MiniGameState::Duel).with_system(update))
        .add_system_set(SystemSet::on_update(MiniGameState::Duel).with_system(ai));
    }
}

pub fn init(
    game: Res<Game>,
    mini_game: Res<MiniGame>,
    mut duel: ResMut<Duel>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    difficulty: Res<Difficulty>,
) {
    duel.time = match difficulty.as_ref() {
        Difficulty::Normal => 45.,
        Difficulty::Hard => 30.,
    };
    duel.coin_penalty = match difficulty.as_ref() {
        Difficulty::Normal => 3,
        Difficulty::Hard => 2,
    };
    duel.player_cooldown_percent = match difficulty.as_ref() {
        Difficulty::Normal => 0.5,
        Difficulty::Hard => 1.,
    };
    if mini_game.practice {
        duel.my_coins = 100;
        duel.your_coins = 100;
    } else {
        duel.my_coins = game.my_coins;
        duel.your_coins = game.your_coins;
    }
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: asset_library.texture_atlas("duel"),
            transform: Transform::from_xyz(-25., 0., 0.),
            sprite: TextureAtlasSprite {
                color: game.your_color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Duelist {
            player: true,
            direction: 1.,
            x: -25.,
            ..Default::default()
        })
        .insert(MiniGameEntity);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: asset_library.texture_atlas("duel"),
            sprite: TextureAtlasSprite {
                flip_x: true,
                color: game.my_color,
                ..Default::default()
            },
            transform: Transform::from_xyz(25., 0., 0.),
            ..Default::default()
        })
        .insert(Duelist {
            player: false,
            direction: -1.,
            x: 25.,
            ..Default::default()
        })
        .insert(MiniGameEntity);
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        position: Rect {
                            top: Val::Px(0.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "",
                        TextStyle {
                            font: asset_library.font("game"),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Left,
                            vertical: VerticalAlign::Top,
                        },
                    ),
                    ..Default::default()
                })
                .insert(DuelHud);
        })
        .insert(MiniGameEntity);
}

pub fn update(
    game: Res<Game>,
    mut duel: ResMut<Duel>,
    input: Res<Input<KeyCode>>,
    mut duelist_query: Query<(&mut Duelist, &mut Transform, &mut TextureAtlasSprite)>,
    time: Res<Time>,
    mut mini_game_finish: EventWriter<MiniGameFinish>,
    mut mini_game: ResMut<MiniGame>,
    mut hud_query: Query<&mut Text, With<DuelHud>>,
    audio: Res<Audio>,
    asset_library: Res<AssetLibrary>,
    difficulty: Res<Difficulty>,
) {
    if !mini_game.active {
        for mut text in hud_query.iter_mut() {
            text.sections[0].value = "".into();
        }
        return;
    }
    let mut player_attacking = false;
    let mut player_defend_time = 0.;
    let mut player_hit = false;
    let mut player_stun = false;
    let mut player_defended = false;
    let mut myself_attacking = false;
    let mut myself_defend_time = 0.;
    let mut myself_hit = false;
    let mut myself_stun = false;
    let mut myself_defended = false;
    for (mut duelist, mut transform, mut sprite) in duelist_query.iter_mut() {
        let attack_prep_time = if duelist.player {
            match *difficulty {
                Difficulty::Normal => ATTACK_PREP_TIME * 0.5,
                Difficulty::Hard => ATTACK_PREP_TIME,
            }
        } else {
            ATTACK_PREP_TIME
        };
        if duelist.player {
            if input.just_pressed(KeyCode::A) {
                duelist.wants_to_attack = true;
            }
            if input.just_pressed(KeyCode::D) {
                duelist.wants_to_defend = true;
            }
            if !input.pressed(KeyCode::A) {
                duelist.wants_to_attack = false;
            }
            if !input.pressed(KeyCode::D) {
                duelist.wants_to_defend = false;
            }
        }
        if duelist.attack_time >= 0.3 {
            duelist.wants_to_attack = true;
        }
        if duelist.defend_time > 0. && !duelist.wants_to_defend {
            if duelist.player {
                duelist.cooldown = COOLDOWN * duel.player_cooldown_percent;
            } else {
                duelist.cooldown = COOLDOWN;
            }
        }
        if duelist.cooldown <= 0. {
            duelist.attacking = duelist.wants_to_attack;
            duelist.defending = duelist.wants_to_defend;
        } else {
            duelist.wants_to_attack = false;
            duelist.wants_to_defend = false;
            duelist.cooldown -= time.delta_seconds();
            duelist.attacking = false;
            duelist.defending = false;
        }
        if duelist.attacking {
            duelist.defend_time = 0.;
            duelist.attack_time += time.delta_seconds();
        } else if duelist.defending {
            duelist.attack_time = 0.;
            duelist.defend_time += time.delta_seconds();
            if duelist.defend_time > PARRY_TIME {
                duelist.wants_to_defend = false;
            }
        } else {
            duelist.attack_time = 0.;
            duelist.defend_time = 0.;
        }
        if duelist.cooldown <= 0. {
            duelist.hit = false;
        }
        if duelist.attack_time == 0. {
            duelist.attacked = false;
        }
        sprite.color.set_a(1.);
        if duelist.cooldown > 0. {
            sprite.color.set_a(0.3);
            if duelist.hit {
                transform.translation.x = duelist.x + duelist.direction * -3.;
                sprite.index = 5;
            } else {
                transform.translation.x = duelist.x;
                sprite.index = 3;
            }
        } else if duelist.attack_time > 0. {
            if duelist.attack_time < attack_prep_time {
                transform.translation.x = duelist.x;
                sprite.index = 1;
            } else if duelist.attack_time < attack_prep_time + ATTACK_STAB_TIME {
                if duelist.attack_time > attack_prep_time + ATTACK_STAB_TIME * 0.5
                    && !duelist.attacked
                {
                    duelist.attacked = true;
                    if duelist.player {
                        player_attacking = true;
                    } else {
                        myself_attacking = true;
                    }
                }
                transform.translation.x = duelist.x + 10.;
                sprite.index = 2;
            } else {
                if duelist.player {
                    duelist.cooldown = COOLDOWN * duel.player_cooldown_percent;
                } else {
                    duelist.cooldown = COOLDOWN;
                }
            }
        } else if duelist.defend_time > 0. {
            if duelist.player {
                player_defend_time = duelist.defend_time;
            } else {
                myself_defend_time = duelist.defend_time;
            }
            transform.translation.x = duelist.x + duelist.direction * -5.;
            sprite.index = 4;
        } else {
            transform.translation.x = duelist.x;
            sprite.index = 0;
        }
    }
    if player_attacking {
        if myself_defend_time == 0. {
            audio.play(asset_library.audio("duelhit"));
            myself_hit = true;
        } else {
            audio.play(asset_library.audio("duelblock"));
            if myself_defend_time < PARRY_TIME {
                myself_defended = true;
                player_stun = true;
            }
        }
    }
    if myself_attacking {
        if player_defend_time == 0. {
            audio.play(asset_library.audio("duelhit"));
            player_hit = true;
        } else {
            audio.play(asset_library.audio("duelblock"));
            if player_defend_time < PARRY_TIME {
                player_defended = true;
                myself_stun = true;
            }
        }
    }
    for (mut duelist, _, _) in duelist_query.iter_mut() {
        let hit = if duelist.player {
            player_hit
        } else {
            myself_hit
        };
        let stun = if duelist.player {
            player_stun
        } else {
            myself_stun
        };
        let defended = if duelist.player {
            player_defended
        } else {
            myself_defended
        };
        if hit || stun {
            duelist.attack_time = 0.;
            duelist.defend_time = 0.;
            if hit {
                duelist.hit = true;
                if duelist.player {
                    duelist.cooldown = COOLDOWN * 0.75 * duel.player_cooldown_percent;
                } else {
                    duelist.cooldown = COOLDOWN * 0.75;
                }
            }
            if stun {
                if duelist.player {
                    duelist.cooldown = COOLDOWN * 2. * duel.player_cooldown_percent;
                } else {
                    duelist.cooldown = COOLDOWN * 2.;
                }
            }
        }
        if defended {
            duelist.wants_to_defend = false;
            duelist.defend_time = 0.;
        }
    }
    if player_hit && duel.your_coins > 0 {
        if duel.coin_penalty > duel.your_coins {
            duel.your_coins = 0;
            duel.my_coins += 1;
        } else {
            duel.your_coins -= duel.coin_penalty;
            duel.my_coins += 1;
        }
    }
    if myself_hit && duel.my_coins > 0 {
        if duel.coin_penalty > duel.my_coins {
            duel.my_coins = 0;
            duel.your_coins += 1;
        } else {
            duel.my_coins -= duel.coin_penalty;
            duel.your_coins += 1;
        }
    }
    duel.time -= time.delta_seconds();
    if duel.time <= 0. || duel.your_coins == 0 || duel.my_coins == 0 {
        mini_game_finish.send(MiniGameFinish {
            my_coins: duel.my_coins as i32 - game.my_coins as i32,
            your_coins: duel.your_coins as i32 - game.your_coins as i32,
        });
    }
    for mut text in hud_query.iter_mut() {
        text.sections[0].value = format!("{:.0}", duel.time)
    }
    mini_game.display_my_coins = duel.my_coins;
    mini_game.display_your_coins = duel.your_coins;
}

pub fn ai(
    mini_game: Res<MiniGame>,
    mut duelist_query: Query<&mut Duelist>,
    difficulty: Res<Difficulty>,
) {
    if !mini_game.active {
        return;
    }
    let attack_chance: f32 = match difficulty.as_ref() {
        Difficulty::Normal => 0.03,
        Difficulty::Hard => 0.03,
    };
    let attack_stop_chance: f32 = match difficulty.as_ref() {
        Difficulty::Normal => 0.005,
        Difficulty::Hard => 0.03,
    };
    let mut duelist_count = 0;
    let mut player_attack_time = 0.;
    let mut player_defend_time = 0.;
    let mut player_cooldown = 0.;
    for duelist in duelist_query.iter() {
        duelist_count += 1;
        if duelist.player {
            player_attack_time = duelist.attack_time;
            player_defend_time = duelist.defend_time;
            player_cooldown = duelist.defend_time;
        }
    }
    if duelist_count == 2 {
        let mut rng = rand::thread_rng();
        for mut duelist in duelist_query.iter_mut() {
            if !duelist.player {
                if !duelist.wants_to_attack && !duelist.wants_to_defend {
                    if player_attack_time < ATTACK_PREP_TIME * 0.5 {
                        let cooldown_add = if player_cooldown > 0. { 0.05 } else { 0. };
                        let attack_chance =
                            (attack_chance + (player_defend_time / 10.0f32) + cooldown_add)
                                .clamp(0., 1.);
                        if rng.gen_bool(attack_chance as f64) {
                            duelist.wants_to_attack = true;
                        }
                    }
                    if !duelist.wants_to_attack && player_attack_time > 0. {
                        let defend_chance = (0.01f32 + (player_attack_time / 5.0f32)).clamp(0., 1.);
                        if rng.gen_bool(defend_chance as f64) {
                            duelist.wants_to_defend = true;
                        }
                    }
                } else if duelist.wants_to_attack {
                    let distance_from_midpoint =
                        (duelist.attack_time - ATTACK_PREP_TIME * 0.5).abs();
                    let chance_to_stop_attack = (attack_stop_chance
                        - attack_stop_chance * distance_from_midpoint.powf(3.))
                    .clamp(0., 1.);
                    if rng.gen_bool(chance_to_stop_attack as f64) {
                        duelist.wants_to_attack = false;
                    }
                }
            }
        }
    }
}
