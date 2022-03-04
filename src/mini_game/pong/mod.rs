use crate::prelude::*;
use bevy::prelude::*;
use paddle::{Paddle, PaddleAi, PaddlePlayer, PaddlePlugin};
use puck::{Puck, PuckPlugin};

#[derive(Default, Component)]
pub struct Pong {
    started: bool,
    finished: bool,
    spawn_puck: bool,
    spawn_time: f32,
    spawn_count: u32,
    my_coins: u32,
    your_coins: u32,
}

pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PaddlePlugin)
            .add_plugin(PuckPlugin)
            .insert_resource(Pong::default())
            .add_system_set(SystemSet::on_enter(MiniGameState::Pong).with_system(init))
            .add_system_set(SystemSet::on_update(MiniGameState::Pong).with_system(update));
    }
}

pub fn init(
    game: Res<Game>,
    mut pong: ResMut<Pong>,
    mut commands: Commands,
    mini_game: Res<MiniGame>,
) {
    *pong = Pong::default();
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Vec2::new(48., 8.).into(),
                color: game.your_color,
                ..Default::default()
            },
            transform: Transform::from_xyz(0., -80., 0.),
            ..Default::default()
        })
        .insert(Paddle)
        .insert(Collision {
            shape: CollisionShape::Rect {
                size: Vec2::new(48., 8.),
            },
            flags: 0x1000,
        })
        .insert(PaddlePlayer)
        .insert(MiniGameEntity);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Vec2::new(48., 8.).into(),
                color: game.my_color,
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 80., 0.),
            ..Default::default()
        })
        .insert(Paddle)
        .insert(Collision {
            shape: CollisionShape::Rect {
                size: Vec2::new(48., 8.),
            },
            flags: 0x1000,
        })
        .insert(PaddleAi::default())
        .insert(MiniGameEntity);
    if !mini_game.practice {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Vec2::new(48., 8.).into(),
                    color: game.my_color,
                    ..Default::default()
                },
                transform: Transform::from_xyz(0., 80., 0.),
                ..Default::default()
            })
            .insert(Paddle)
            .insert(Collision {
                shape: CollisionShape::Rect {
                    size: Vec2::new(48., 8.),
                },
                flags: 0x1000,
            })
            .insert(PaddleAi::default())
            .insert(MiniGameEntity);
    }
}

pub fn update(
    mut pong: ResMut<Pong>,
    mut mini_game: ResMut<MiniGame>,
    mut commands: Commands,
    time: Res<Time>,
    mut mini_game_finish: EventWriter<MiniGameFinish>,
) {
    if mini_game.active && !pong.started {
        pong.spawn_puck = true;
        pong.started = true;
    }
    if mini_game.active {
        pong.spawn_time += time.delta_seconds();
    }
    if pong.spawn_time > 10. && pong.spawn_count < 2 {
        pong.spawn_puck = true;
        pong.spawn_time = 0.;
        pong.spawn_count += 1;
    }
    if pong.spawn_puck {
        pong.spawn_puck = false;
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Vec2::new(8., 8.).into(),
                    color: Color::WHITE,
                    ..Default::default()
                },
                transform: Transform::from_xyz(0., 0., 0.),
                ..Default::default()
            })
            .insert(Puck {
                ..Default::default()
            })
            .insert(Collision {
                shape: CollisionShape::Rect {
                    size: Vec2::new(8., 8.),
                },
                flags: 0x0100,
            })
            .insert(MiniGameEntity);
    }
    if pong.my_coins + pong.your_coins == 6 && !pong.finished {
        mini_game_finish.send(MiniGameFinish {
            my_coins: pong.my_coins as i32,
            your_coins: pong.your_coins as i32,
        });
    }
    mini_game.display_my_coins = pong.my_coins as u32;
    mini_game.display_your_coins = pong.your_coins as u32;
}

pub mod paddle;
pub mod puck;
