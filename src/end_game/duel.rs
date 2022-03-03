use super::prelude::*;
use crate::mini_game::duel::ATTACK_PREP_TIME;
use crate::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::Audio;

pub struct EgDuel {
    attack_time: f32,
    hit: bool,
}

#[derive(Component)]
pub struct EgDuelist;

#[derive(Component)]
pub struct EgMe;

#[derive(Component)]
pub struct EgMeEye;

pub struct EgDuelPlugin;

impl Plugin for EgDuelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EgDuel {
            attack_time: 0.0,
            hit: false,
        })
        .add_system_set(SystemSet::on_enter(EndGameState::Duel).with_system(enter))
        .add_system_set(SystemSet::on_exit(EndGameState::Duel).with_system(exit))
        .add_system_set(SystemSet::on_update(EndGameState::Duel).with_system(update))
        .add_system_set(SystemSet::on_update(EndGameState::Duel).with_system(update_me))
        .add_system_set(SystemSet::on_update(EndGameState::Duel).with_system(update_me_eye));
    }
}

pub fn enter(
    mut eg_duel: ResMut<EgDuel>,
    game: Res<Game>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    eg_duel.attack_time = 0.;
    eg_duel.hit = false;
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: asset_library.texture_atlas("duel"),
            transform: Transform::from_xyz(-25., 0., 0.2),
            sprite: TextureAtlasSprite {
                color: game.your_color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(EndGameEntity)
        .insert(EgDuelist);
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_library.image("me_1"),
            transform: Transform::from_xyz(55., 0., 0.),
            sprite: Sprite {
                color: game.my_color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(EgMe)
        .insert(EndGameEntity);
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_library.image("me_2"),
            transform: Transform::from_xyz(55., 0., 0.1),
            sprite: Sprite {
                color: game.my_color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(EgMeEye)
        .insert(EndGameEntity);
}

pub fn exit(mut commands: Commands, query: Query<Entity, With<EndGameEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn update(
    mut end_game: ResMut<EndGame>,
    mut eg_duel: ResMut<EgDuel>,
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut TextureAtlasSprite), With<EgDuelist>>,
    time: Res<Time>,
    asset_library: Res<AssetLibrary>,
    audio: Res<Audio>,
) {
    if input.pressed(KeyCode::A) || eg_duel.attack_time >= ATTACK_PREP_TIME {
        eg_duel.attack_time += time.delta_seconds();
    }
    let duelist_x = -25.;
    for (mut transform, mut sprite) in query.iter_mut() {
        transform.translation.x = duelist_x;
        if eg_duel.attack_time == 0. {
            sprite.index = 0;
        } else if eg_duel.attack_time < ATTACK_PREP_TIME {
            sprite.index = 1;
        } else {
            if !eg_duel.hit {
                end_game.my_health -= 1;
                audio.play(asset_library.audio("duelhit"));
                eg_duel.hit = true;
            }
            transform.translation.x = duelist_x + 10.;
            sprite.index = 2;
        }
    }
    if end_game.state_time > 2. {
        end_game.switch = true;
    }
}

pub fn update_me(
    end_game: Res<EndGame>,
    eg_duel: Res<EgDuel>,
    mut query: Query<(&mut Transform, &mut Sprite), With<EgMe>>,
) {
    let t = end_game.state_time * 2.;
    for (mut transform, mut sprite) in query.iter_mut() {
        transform.translation.y = 10. + t.cos() * 2.;
        if eg_duel.hit {
            sprite.color.set_a(0.3);
        } else {
            sprite.color.set_a(1.);
        }
    }
}

pub fn update_me_eye(
    end_game: Res<EndGame>,
    eg_duel: Res<EgDuel>,
    mut query: Query<(&mut Transform, &mut Sprite), With<EgMeEye>>,
) {
    let t = end_game.state_time * 2.;
    for (mut transform, mut sprite) in query.iter_mut() {
        transform.translation.y = 10. + t.cos() * 1.8;
        transform.translation.x = 55. + t.sin() * 2.;
        if eg_duel.hit {
            sprite.color.set_a(0.3);
        } else {
            sprite.color.set_a(1.);
        }
    }
}
