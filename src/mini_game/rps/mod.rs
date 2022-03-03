use crate::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use rand::Rng;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RpsSelect {
    Rock,
    Paper,
    Scissors,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RpsCompare {
    Win,
    Lose,
    Draw,
}

impl RpsSelect {
    pub fn new_rand() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..=2) {
            0 => RpsSelect::Rock,
            1 => RpsSelect::Paper,
            _ => RpsSelect::Scissors,
        }
    }
    pub fn new_winner(other: RpsSelect) -> Self {
        match other {
            RpsSelect::Rock => RpsSelect::Paper,
            RpsSelect::Paper => RpsSelect::Scissors,
            RpsSelect::Scissors => RpsSelect::Rock,
        }
    }
    pub fn to_index(&self) -> usize {
        match *self {
            RpsSelect::Rock => 0,
            RpsSelect::Paper => 1,
            RpsSelect::Scissors => 2,
        }
    }
    pub fn compare(&self, other: RpsSelect) -> RpsCompare {
        match *self {
            RpsSelect::Rock => match other {
                RpsSelect::Rock => RpsCompare::Draw,
                RpsSelect::Paper => RpsCompare::Lose,
                RpsSelect::Scissors => RpsCompare::Win,
            },
            RpsSelect::Paper => match other {
                RpsSelect::Rock => RpsCompare::Win,
                RpsSelect::Paper => RpsCompare::Draw,
                RpsSelect::Scissors => RpsCompare::Lose,
            },
            RpsSelect::Scissors => match other {
                RpsSelect::Rock => RpsCompare::Lose,
                RpsSelect::Paper => RpsCompare::Win,
                RpsSelect::Scissors => RpsCompare::Draw,
            },
        }
    }
}

pub enum RpsState {
    Countdown {
        time: f32,
        stage: u32,
        y: f32,
        can_advance: bool,
    },
    Play {
        time: f32,
        my_selection: RpsSelect,
        your_selection: RpsSelect,
        selection_window: f32,
    },
}

#[derive(Component)]
pub struct RpsController {
    state: RpsState,
    rounds: u32,
    my_coins: u32,
    your_coins: u32,
}

#[derive(Component)]
pub struct RpsHand {
    index: u32,
}

#[derive(Component)]
pub struct RpsText;

pub struct RpsPlugin;

impl Plugin for RpsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(MiniGameState::Rps).with_system(rps_init))
            .add_system_set(SystemSet::on_update(MiniGameState::Rps).with_system(rps_update))
            .add_system_set(SystemSet::on_update(MiniGameState::Rps).with_system(rps_update_coins))
            .add_system_set(SystemSet::on_update(MiniGameState::Rps).with_system(rps_input));
    }
}

pub fn rps_init(
    game: Res<Game>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn()
        .insert(RpsController {
            state: RpsState::Countdown {
                time: 0.0,
                stage: 0,
                y: 0.0,
                can_advance: true,
            },
            rounds: 3,
            my_coins: 0,
            your_coins: 0,
        })
        .insert(MiniGameEntity);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: asset_library.texture_atlas("rps"),
            transform: Transform::from_xyz(-60., 0., 0.3),
            sprite: TextureAtlasSprite {
                color: game.your_color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RpsHand { index: 1 })
        .insert(MiniGameEntity);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: asset_library.texture_atlas("rps"),
            transform: Transform::from_xyz(60., 0., 0.3),
            sprite: TextureAtlasSprite {
                color: game.my_color,
                flip_x: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RpsHand { index: 0 })
        .insert(MiniGameEntity);
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            transform: Transform::from_xyz(0.0, 65.0, 0.5),
            ..Default::default()
        })
        .insert(RpsText)
        .insert(MiniGameEntity);
}

pub fn rps_update(
    game: Res<Game>,
    mut query: Query<&mut RpsController>,
    mut hands: Query<(&mut Transform, &mut TextureAtlasSprite, &RpsHand)>,
    mut text_query: Query<&mut Text, With<RpsText>>,
    mut mini_game_finish: EventWriter<MiniGameFinish>,
    timer: Res<Time>,
    mini_game: Res<MiniGame>,
    difficulty: Res<Difficulty>,
    audio: Res<Audio>,
    asset_library: Res<AssetLibrary>,
) {
    let reshoot_chance = if mini_game.practice {
        0.
    } else if game.turn == 2 {
        0.6
    } else {
        match *difficulty {
            Difficulty::Easy => 0.3,
            Difficulty::Normal => 0.4,
            Difficulty::Hard => 0.6,
            Difficulty::VeryHard => 0.6,
        }
    };
    if !mini_game.active {
        return;
    }
    let mut rng = rand::thread_rng();
    for mut controller in query.iter_mut() {
        let RpsController {
            state,
            your_coins,
            my_coins,
            rounds,
            ..
        } = controller.as_mut();
        for (mut hand_transform, mut sprite, hand) in hands.iter_mut() {
            let selection = match state {
                RpsState::Play {
                    my_selection,
                    your_selection,
                    ..
                } => {
                    if hand.index == 0 {
                        Some(*my_selection)
                    } else if hand.index == 1 {
                        Some(*your_selection)
                    } else {
                        None
                    }
                }
                _ => Some(RpsSelect::Rock),
            };
            match state {
                RpsState::Countdown { .. } => {
                    sprite.color.set_a(0.3);
                }
                RpsState::Play {
                    selection_window, ..
                } => {
                    if hand.index == 0 {
                        sprite.color.set_a(1.0);
                    } else if *selection_window > 0. {
                        sprite.color.set_a(0.3);
                    } else {
                        sprite.color.set_a(1.0);
                    }
                }
            }
            if let Some(selection) = selection {
                sprite.index = selection.to_index();
            }
            let y = match state {
                RpsState::Countdown { y, .. } => *y * 10.,
                _ => 0.,
            };
            hand_transform.translation.y = y;
        }
        match state {
            RpsState::Countdown {
                time,
                stage,
                y,
                can_advance,
            } => {
                *time += timer.delta_seconds();
                *y = (*time * 15.).cos();
                if *y > 0.8 && *can_advance {
                    *stage += 1;
                    *can_advance = false;
                    match *stage {
                        1 => {
                            audio.play(asset_library.audio("rock"));
                        }
                        2 => {
                            audio.play(asset_library.audio("paper"));
                        }
                        3 => {
                            audio.play(asset_library.audio("scissors"));
                        }
                        4 => {
                            audio.play(asset_library.audio("shoot"));
                        }
                        _ => {}
                    }
                }
                if *y < 0. {
                    *can_advance = true;
                }
                if *stage == 4 {
                    *state = RpsState::Play {
                        time: 0.0,
                        my_selection: RpsSelect::new_rand(),
                        your_selection: RpsSelect::Rock,
                        selection_window: 1.0,
                    };
                }
            }
            RpsState::Play {
                time,
                my_selection,
                your_selection,
                selection_window,
                ..
            } => {
                *time += timer.delta_seconds();
                *selection_window -= timer.delta_seconds();
                if *time > 2. {
                    let losing = my_selection.compare(*your_selection) == RpsCompare::Lose;
                    if losing && rng.gen_bool(reshoot_chance) {
                        audio.play(asset_library.audio("shoot"));
                        *time = 0.0;
                        let old_selection = *my_selection;
                        while *my_selection == old_selection {
                            if rng.gen_bool(0.5) {
                                *my_selection = *your_selection;
                            } else {
                                *my_selection = RpsSelect::new_winner(*your_selection);
                            }
                        }
                        *selection_window = 1.0;
                    } else {
                        match my_selection.compare(*your_selection) {
                            RpsCompare::Win => {
                                *my_coins += 2;
                                *rounds -= 1;
                                *your_selection = RpsSelect::Rock;
                                *my_selection = RpsSelect::Rock;
                            }
                            RpsCompare::Lose => {
                                *your_coins += 2;
                                *rounds -= 1;
                                *your_selection = RpsSelect::Rock;
                                *my_selection = RpsSelect::Rock;
                            }
                            RpsCompare::Draw => {}
                        }
                        if *rounds > 0 {
                            *state = RpsState::Countdown {
                                time: 0.0,
                                stage: 0,
                                y: 0.0,
                                can_advance: true,
                            };
                        } else {
                            mini_game_finish.send(MiniGameFinish {
                                my_coins: *my_coins as i32,
                                your_coins: *your_coins as i32,
                            });
                        }
                    }
                }
            }
        }
        for mut text in text_query.iter_mut() {
            match state {
                RpsState::Countdown { stage, .. } => {
                    if *stage == 0 {
                        text.sections[0].value = "".into();
                    } else if *stage == 1 {
                        text.sections[0].value = "Rock".into();
                    } else if *stage == 2 {
                        text.sections[0].value = "Paper".into();
                    } else if *stage == 3 {
                        text.sections[0].value = "Scissors".into();
                    }
                }
                RpsState::Play {
                    selection_window, ..
                } => {
                    if *selection_window > 0. {
                        text.sections[0].value = "Shoot!".into();
                    } else {
                        text.sections[0].value = "".into();
                    }
                }
            }
        }
    }
}

pub fn rps_update_coins(mut query: Query<&mut RpsController>, mut mini_game: ResMut<MiniGame>) {
    for mut controller in query.iter_mut() {
        let RpsController {
            your_coins,
            my_coins,
            ..
        } = controller.as_mut();
        mini_game.display_your_coins = *your_coins;
        mini_game.display_my_coins = *my_coins;
    }
}

pub fn rps_input(input: Res<Input<KeyCode>>, mut query: Query<&mut RpsController>) {
    for mut controller in query.iter_mut() {
        let RpsController { state, .. } = controller.as_mut();
        if let RpsState::Play {
            your_selection,
            selection_window,
            ..
        } = state
        {
            if *selection_window > 0. {
                if input.just_pressed(KeyCode::R) {
                    *your_selection = RpsSelect::Rock;
                    *selection_window = 0.;
                } else if input.just_pressed(KeyCode::P) {
                    *your_selection = RpsSelect::Paper;
                    *selection_window = 0.;
                } else if input.just_pressed(KeyCode::S) {
                    *your_selection = RpsSelect::Scissors;
                    *selection_window = 0.;
                }
            }
        }
    }
}
