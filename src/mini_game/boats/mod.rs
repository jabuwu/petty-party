use crate::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use boat::{Boat, BoatPlugin};
use cannon_ball::{CannonBall, CannonBallPlugin};
use enemy_boat::{EnemyBoat, EnemyBoatPlugin};
use player_boat::{PlayerBoat, PlayerBoatPlugin};
use rand::prelude::*;

pub struct BoatsPlugin;

impl Plugin for BoatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CannonBallPlugin)
            .add_plugin(BoatPlugin)
            .add_plugin(PlayerBoatPlugin)
            .add_plugin(EnemyBoatPlugin)
            .add_system_set(SystemSet::on_enter(MiniGameState::Boats).with_system(enter))
            .add_system_set(SystemSet::on_update(MiniGameState::Boats).with_system(spawn_cannons))
            .add_system_set(SystemSet::on_update(MiniGameState::Boats).with_system(update_coins));
    }
}

pub fn enter(
    mut mini_game: ResMut<MiniGame>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    difficulty: Res<Difficulty>,
) {
    mini_game.display_prefix = "+".into();
    let my_size = if mini_game.practice {
        1.
    } else {
        match *difficulty {
            Difficulty::Normal => 0.75,
            Difficulty::Hard => 0.5,
        }
    };
    let your_size = if mini_game.practice {
        1.
    } else {
        match *difficulty {
            Difficulty::Normal => 1.2,
            Difficulty::Hard => 1.2,
        }
    };
    let player_hitbox_size = 0.8;
    let my_hitbox_size = match *difficulty {
        Difficulty::Normal => 1.2,
        Difficulty::Hard => 0.8,
    };
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_library.image("boats_bg"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(MiniGameEntity);

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_library.image("boat"),
            transform: Transform::from_xyz(-100.0, 0.0, 0.1)
                .with_scale(Vec3::new(your_size, your_size, 1.)),
            ..Default::default()
        })
        .insert(Collision {
            shape: CollisionShape::Rect {
                size: Vec2::new(
                    50. * your_size * player_hitbox_size,
                    70. * your_size * player_hitbox_size,
                ),
            },
            flags: 0x1000,
        })
        .insert(Boat {
            my_boat: false,
            coins: 6,
            ..Default::default()
        })
        .insert(PlayerBoat)
        .insert(MiniGameEntity);

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_library.image("boat"),
            transform: Transform::from_xyz(100.0, 0.0, 0.1)
                .with_scale(Vec3::new(my_size, my_size, 1.)),
            ..Default::default()
        })
        .insert(Collision {
            shape: CollisionShape::Rect {
                size: Vec2::new(
                    59. * my_size * my_hitbox_size,
                    70. * my_size * my_hitbox_size,
                ),
            },
            flags: 0x1000,
        })
        .insert(Boat {
            my_boat: true,
            coins: 6,
            ..Default::default()
        })
        .insert(EnemyBoat::default())
        .insert(MiniGameEntity);
}

pub fn spawn_cannons(
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    mini_game: Res<MiniGame>,
    difficulty: Res<Difficulty>,
    audio: Res<Audio>,
) {
    if !mini_game.active {
        return;
    }
    let spawn_chance = match *difficulty {
        Difficulty::Normal => 0.05,
        Difficulty::Hard => 0.08,
    };
    let mut rng = rand::thread_rng();
    if rng.gen_bool(spawn_chance) {
        audio.play(asset_library.audio("cannon"));
        let angle = rng.gen_range(0.0..360.0f32).to_radians();
        let x = angle.cos() * 160.;
        let y = angle.sin() * 140.;
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(1., 1., 1., 0.),
                    ..Default::default()
                },
                texture: asset_library.image("cannon_shadow"),
                transform: Transform::from_xyz(x, y, 0.2),
                ..Default::default()
            })
            .insert(Collision {
                shape: CollisionShape::Rect {
                    size: Vec2::new(20.0, 20.0),
                },
                flags: 0x0100,
            })
            .insert(CannonBall {
                falling_size: 0.0,
                landing_size: 0.0,
                speed: 0.1,
                dir: Vec2::ZERO,
                dir_set: false,
            })
            .insert(MiniGameEntity);
    }
}

pub fn update_coins(
    mut mini_game: ResMut<MiniGame>,
    boat_query: Query<&Boat>,
    mut mini_game_finish: EventWriter<MiniGameFinish>,
) {
    let mut boats_initialized = false;
    let mut my_coins = 0;
    let mut your_coins = 0;
    for boat in boat_query.iter() {
        boats_initialized = true;
        if boat.my_boat {
            my_coins = boat.coins;
            mini_game.display_my_coins = my_coins;
        } else {
            your_coins = boat.coins;
            mini_game.display_your_coins = your_coins;
        }
    }
    if boats_initialized && (my_coins == 0 || your_coins == 0) {
        mini_game_finish.send(MiniGameFinish {
            my_coins: my_coins as i32,
            your_coins: your_coins as i32,
        });
    }
}

mod boat;
mod cannon_ball;
mod enemy_boat;
mod player_boat;
