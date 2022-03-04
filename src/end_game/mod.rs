use crate::prelude::*;
use bevy::prelude::*;
use boats::EgBoatsPlugin;
use duel::EgDuelPlugin;
use noise::NoisePlugin;
use pong::EgPongPlugin;
use rps::EgRpsPlugin;

#[derive(Component)]
pub struct EndGameEntity;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum EndGameState {
    Inactive,
    Noise,
    Rps,
    Duel,
    Boats,
    Pong,
}

pub struct EndGame {
    turn: u32,
    state_time: f32,
    switch: bool,
    my_health: u32,
    your_health: u32,
}

#[derive(Component)]
pub struct EndGameHeart {
    mine: bool,
    amount: u32,
}

pub struct EndGamePlugin;

impl Plugin for EndGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(NoisePlugin)
            .add_plugin(EgRpsPlugin)
            .add_plugin(EgDuelPlugin)
            .add_plugin(EgBoatsPlugin)
            .add_plugin(EgPongPlugin)
            .add_state(EndGameState::Inactive)
            .insert_resource(EndGame {
                turn: 0,
                state_time: 0.,
                switch: false,
                my_health: 3,
                your_health: 3,
            })
            .add_system_set(SystemSet::on_enter(GameState::EndGame).with_system(enter))
            .add_system_set(SystemSet::on_exit(GameState::EndGame).with_system(exit))
            .add_system_set(SystemSet::on_update(GameState::EndGame).with_system(update));
    }
}

pub fn enter(
    game: Res<Game>,
    mut end_game_state: ResMut<State<EndGameState>>,
    mut end_game: ResMut<EndGame>,
    mut camera_controller: ResMut<CameraController>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    let _ = end_game_state.set(EndGameState::Noise);
    end_game.turn = 0;
    end_game.state_time = 0.0;
    end_game.switch = false;
    end_game.my_health = 3;
    end_game.your_health = 3;
    camera_controller.center = true;
    camera_controller.follow_entity = None;
    for i in 0..3 {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_library.image("heart"),
                transform: Transform::from_xyz(-135. + 40. * (i as f32), 100., 1.),
                sprite: Sprite {
                    color: game.your_color,
                    ..Default::default()
                },
                visibility: Visibility { is_visible: false },
                ..Default::default()
            })
            .insert(EndGameHeart {
                mine: false,
                amount: i + 1,
            });
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_library.image("heart"),
                transform: Transform::from_xyz(135. - 40. * (i as f32), 100., 1.),
                sprite: Sprite {
                    color: game.my_color,
                    ..Default::default()
                },
                visibility: Visibility { is_visible: false },
                ..Default::default()
            })
            .insert(EndGameHeart {
                mine: true,
                amount: i + 1,
            });
    }
}
pub fn exit(
    mut end_game_state: ResMut<State<EndGameState>>,
    query: Query<Entity, With<EndGameHeart>>,
    mut commands: Commands,
) {
    let _ = end_game_state.set(EndGameState::Inactive);
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn update(
    mut end_game_state: ResMut<State<EndGameState>>,
    mut end_game: ResMut<EndGame>,
    time: Res<Time>,
    mut query: Query<(&mut Handle<Image>, &mut Visibility, &EndGameHeart)>,
    asset_library: Res<AssetLibrary>,
    mut reset: EventWriter<GameResetSend>,
    mut game_state: ResMut<State<GameState>>,
    difficulty: Res<Difficulty>,
) {
    end_game.state_time += time.delta_seconds();
    if matches!(end_game_state.current(), EndGameState::Noise) {
        if end_game.my_health == 0 {
            if end_game.state_time > 2.0 {
                game_state.set(GameState::Ending).unwrap();
                end_game.state_time = 0.;
            }
        } else if end_game.your_health == 0 {
            if end_game.state_time > 2.0 {
                if matches!(*difficulty, Difficulty::Hard) {
                    reset.send(GameResetSend);
                } else {
                    game_state.set(GameState::Continue).unwrap();
                }
                end_game.state_time = 0.;
            }
        } else {
            if end_game.state_time > 0.5 {
                end_game.turn += 1;
                end_game.state_time = 0.;
                match end_game.turn % 5 {
                    0 => end_game_state.set(EndGameState::Rps).unwrap(),
                    1 => end_game_state.set(EndGameState::Boats).unwrap(),
                    2 => end_game_state.set(EndGameState::Rps).unwrap(),
                    3 => end_game_state.set(EndGameState::Pong).unwrap(),
                    _ => end_game_state.set(EndGameState::Duel).unwrap(),
                }
            }
        }
    } else if end_game.switch {
        end_game.state_time = 0.;
        end_game_state.set(EndGameState::Noise).unwrap();
        end_game.switch = false;
    }
    for (mut image, mut visibility, heart) in query.iter_mut() {
        if heart.mine {
            if heart.amount > end_game.my_health {
                *image = asset_library.image("heart_empty");
            } else {
                *image = asset_library.image("heart");
            }
        } else {
            if heart.amount > end_game.your_health {
                *image = asset_library.image("heart_empty");
            } else {
                *image = asset_library.image("heart");
            }
        }
        visibility.is_visible = !matches!(end_game_state.current(), EndGameState::Noise);
    }
}

pub mod boats;
pub mod duel;
pub mod noise;
pub mod pong;
pub mod rps;

pub mod prelude {
    pub use super::{EndGame, EndGameEntity, EndGameState};
}
