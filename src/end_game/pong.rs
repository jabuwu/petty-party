use super::prelude::*;
use crate::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use rand::prelude::*;

pub struct EgPong {
    next_spawn_time: f32,
    lost_health: bool,
}

pub struct EgPongPlugin;

#[derive(Component)]
pub struct EgPaddle;

#[derive(Component)]
pub struct EgPuck {
    pub velocity: Vec2,
}

impl Plugin for EgPongPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EgPong {
            next_spawn_time: 0.,
            lost_health: false,
        })
        .add_system_set(SystemSet::on_enter(EndGameState::Pong).with_system(enter))
        .add_system_set(SystemSet::on_exit(EndGameState::Pong).with_system(exit))
        .add_system_set(SystemSet::on_update(EndGameState::Pong).with_system(update))
        .add_system_set(SystemSet::on_update(EndGameState::Pong).with_system(paddle_update))
        .add_system_set(SystemSet::on_update(EndGameState::Pong).with_system(puck_update));
    }
}

pub fn enter(mut eg_pong: ResMut<EgPong>, game: Res<Game>, mut commands: Commands) {
    eg_pong.next_spawn_time = 0.2;
    eg_pong.lost_health = false;
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
        .insert(EgPaddle)
        .insert(Collision {
            shape: CollisionShape::Rect {
                size: Vec2::new(48., 8.),
            },
            flags: 0x1000,
        })
        .insert(EndGameEntity);
}

pub fn exit(mut commands: Commands, query: Query<Entity, With<EndGameEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn update(
    mut eg_pong: ResMut<EgPong>,
    mut commands: Commands,
    time: Res<Time>,
    game: Res<Game>,
    mut end_game: ResMut<EndGame>,
) {
    let mut spawn = false;
    if eg_pong.next_spawn_time > 0. {
        eg_pong.next_spawn_time -= time.delta_seconds();
        if eg_pong.next_spawn_time <= 0. {
            eg_pong.next_spawn_time = 1.0;
            spawn = true;
        }
    }
    if spawn {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Vec2::new(8., 8.).into(),
                    color: game.my_color,
                    ..Default::default()
                },
                transform: Transform::from_xyz(0., 100., 0.),
                ..Default::default()
            })
            .insert(EgPuck {
                ..Default::default()
            })
            .insert(Collision {
                shape: CollisionShape::Rect {
                    size: Vec2::new(8., 8.),
                },
                flags: 0x0100,
            })
            .insert(EndGameEntity);
    }
    if end_game.state_time > 4.5 {
        end_game.switch = true;
    }
}

pub fn paddle_update(
    mut paddle_query: Query<&mut Transform, With<EgPaddle>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for mut transform in paddle_query.iter_mut() {
        if input.pressed(KeyCode::A) {
            transform.translation.x -= time.delta_seconds() * 250.;
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += time.delta_seconds() * 250.;
        }
        transform.translation.x = transform.translation.x.clamp(-140., 140.);
    }
}

impl Default for EgPuck {
    fn default() -> Self {
        Self {
            velocity: Vec2::new(0., 0.),
        }
    }
}

pub fn puck_update(
    mut query: Query<(Entity, &mut Transform, &Collision, &mut EgPuck)>,
    collision_query: Res<CollisionQuery>,
    time: Res<Time>,
    mut commands: Commands,
    audio: Res<Audio>,
    asset_library: Res<AssetLibrary>,
    mut end_game: ResMut<EndGame>,
    mut eg_pong: ResMut<EgPong>,
) {
    let mut rng = rand::thread_rng();
    for (entity, mut transform, collision, mut puck) in query.iter_mut() {
        let filter = Some(CollisionFilter {
            exclude_entity: entity,
            flags: 0x1000,
        });
        if puck.velocity.length_squared() < 0.01 {
            let angle = -rng.gen_range(50.0..130.0f32).to_radians();
            puck.velocity = Vec2::new(angle.cos(), angle.sin()) * 150.;
        }
        transform.translation += puck.velocity.extend(0.) * time.delta_seconds();
        if transform.translation.x > 155. {
            puck.velocity.x = puck.velocity.x.abs() * -1.;
        }
        if transform.translation.x < -155. {
            puck.velocity.x = puck.velocity.x.abs();
        }
        if transform.translation.y < -100. {
            if !eg_pong.lost_health {
                end_game.your_health -= 1;
                eg_pong.lost_health = true;
            }
            commands.entity(entity).despawn();
        }
        if let Some(result) =
            collision_query.check(transform.translation.truncate(), collision.shape, filter)
        {
            audio.play(asset_library.audio("pong"));
            let magnitude = puck.velocity.length() + 10.;
            let mut difference =
                (transform.translation.truncate() - result.position).normalize_or_zero();
            while collision_query
                .check(transform.translation.truncate(), collision.shape, filter)
                .is_some()
            {
                transform.translation += difference.extend(0.);
            }
            if difference.y.abs() < 0.2 {
                if difference.y < 0. {
                    difference.y -= 1.;
                } else {
                    difference.y += 1.;
                }
                difference = difference.normalize_or_zero();
            }
            difference *= magnitude;
            puck.velocity = difference;
        }
    }
}
