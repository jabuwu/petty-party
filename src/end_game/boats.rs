use super::prelude::*;
use crate::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use rand::prelude::*;

const ACCELERATION: f32 = 90.0;

#[derive(Component, Default)]
pub struct EgBoat {
    pub movement: Vec2,
    pub invulnerable_timer: f32,
    pub my_boat: bool,
}

impl EgBoat {
    pub fn hit(&mut self) -> bool {
        if self.invulnerable_timer == 0. {
            self.invulnerable_timer = 999.;
            true
        } else {
            false
        }
    }
}

pub struct EgBoatsPlugin;

impl Plugin for EgBoatsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(EndGameState::Boats).with_system(enter))
            .add_system_set(SystemSet::on_exit(EndGameState::Boats).with_system(exit))
            .add_system_set(SystemSet::on_update(EndGameState::Boats).with_system(update))
            .add_system_set(SystemSet::on_update(EndGameState::Boats).with_system(boat_update))
            .add_system_set(
                SystemSet::on_update(EndGameState::Boats).with_system(cannon_ball_update),
            )
            .add_system_set(SystemSet::on_update(EndGameState::Boats).with_system(hit_boats))
            .add_system_set(SystemSet::on_update(EndGameState::Boats).with_system(spawn_cannons));
    }
}

pub fn enter(mut commands: Commands, asset_library: Res<AssetLibrary>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_library.image("boats_bg"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(EndGameEntity);

    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_library.image("boat"),
            transform: Transform::from_xyz(0.0, 0.0, 0.1).with_scale(Vec3::new(1., 1., 1.)),
            ..Default::default()
        })
        .insert(Collision {
            shape: CollisionShape::Rect {
                size: Vec2::new(50. * 0.8, 70. * 0.8),
            },
            flags: 0x1000,
        })
        .insert(EgBoat {
            my_boat: false,
            ..Default::default()
        })
        .insert(EndGameEntity);
}

pub fn exit(mut commands: Commands, query: Query<Entity, With<EndGameEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn update(mut end_game: ResMut<EndGame>) {
    if end_game.state_time > 4.5 {
        end_game.switch = true;
    }
}

pub fn boat_update(
    game: Res<Game>,
    mut boat_query: Query<(Entity, &mut Transform, &Collision, &mut EgBoat, &mut Sprite)>,
    collision_query: Res<CollisionQuery>,
    timer: Res<Time>,
    input: Res<Input<KeyCode>>,
) {
    for (_, _, _, _, mut sprite) in boat_query.iter_mut() {
        sprite.color = game.your_color;
    }
    for (entity, mut transform, collision, mut boat, mut sprite) in boat_query.iter_mut() {
        boat.movement = Vec2::new(0., 0.);
        if input.pressed(KeyCode::S) {
            boat.movement.y -= 1.;
        }
        if input.pressed(KeyCode::W) {
            boat.movement.y += 1.;
        }
        if input.pressed(KeyCode::A) {
            boat.movement.x -= 1.;
        }
        if input.pressed(KeyCode::D) {
            boat.movement.x += 1.;
        }
        boat.movement = boat.movement.normalize_or_zero() * 1.5;
        let movement = boat.movement.normalize_or_zero() * 1.5;
        let collision_filter = Some(CollisionFilter {
            exclude_entity: entity,
            flags: 0x1000,
        });
        let mut iterations = 5;
        while let Some(collision) = collision_query.check(
            transform.translation.truncate(),
            collision.shape,
            collision_filter,
        ) {
            let correction =
                (transform.translation.truncate() - collision.position).normalize_or_zero();
            transform.translation.x += correction.x;
            transform.translation.y += correction.y;
            iterations -= 1;
            if iterations == 0 {
                break;
            }
        }
        if collision_query
            .check_moving(
                transform.translation.truncate(),
                Vec2::new(movement.x, 0.),
                collision.shape,
                collision_filter,
            )
            .is_none()
        {
            transform.translation.x += movement.x;
        }
        if collision_query
            .check_moving(
                transform.translation.truncate(),
                Vec2::new(0., movement.y),
                collision.shape,
                collision_filter,
            )
            .is_none()
        {
            transform.translation.y += movement.y;
        }
        transform.translation.x = transform.translation.x.min(120.).max(-110.);
        transform.translation.y = transform.translation.y.min(60.).max(-60.);
        boat.invulnerable_timer = (boat.invulnerable_timer - timer.delta_seconds()).max(0.);
        let mut color = game.your_color;
        if boat.invulnerable_timer > 0. {
            color.set_a(0.2);
        }
        sprite.color = color;
    }
}

#[derive(Component)]
pub struct EgCannonBall {
    pub falling_size: f32,
    pub landing_size: f32,
    pub speed: f32,
    pub dir: Vec2,
    pub dir_set: bool,
}

pub fn cannon_ball_update(
    mut commands: Commands,
    mut query: Query<(Entity, &mut EgCannonBall, &mut Transform, &mut Sprite)>,
    time: Res<Time>,
    audio: Res<Audio>,
    asset_library: Res<AssetLibrary>,
) {
    let mut rng = rand::thread_rng();
    for (entity, mut cannon_ball, mut transform, mut sprite) in query.iter_mut() {
        if !cannon_ball.dir_set {
            let angle = Vec2::angle_between(Vec2::new(-1., 0.), transform.translation.truncate())
                + rng.gen_range(-45.0..45.0f32).to_radians();
            let speed = rng.gen_range(40.0..100.0f32);
            cannon_ball.dir = Vec2::new(angle.cos() * speed, angle.sin() * speed);
            cannon_ball.dir_set = true;
        }
        cannon_ball.dir *= 1. - (0.2 * time.delta_seconds() * time.delta_seconds());
        transform.translation += cannon_ball.dir.extend(0.) * time.delta_seconds();
        cannon_ball.speed += ACCELERATION * time.delta_seconds() * time.delta_seconds() * 0.5;
        if cannon_ball.falling_size < 1. {
            cannon_ball.falling_size += time.delta_seconds() * cannon_ball.speed;
            cannon_ball.falling_size = cannon_ball.falling_size.min(1.);
            transform.scale.x = cannon_ball.falling_size * 0.4;
            transform.scale.y = cannon_ball.falling_size * 0.4;
            sprite.color = Color::rgba(1., 1., 1., cannon_ball.falling_size * 0.4);
        } else if cannon_ball.landing_size < 0.5 {
            cannon_ball.landing_size += time.delta_seconds() * cannon_ball.speed;
            cannon_ball.landing_size = cannon_ball.landing_size.min(1.);
            transform.scale.x = 1. - cannon_ball.landing_size;
            transform.scale.y = 1. - cannon_ball.landing_size;
            sprite.color = Color::rgba(1., 1., 1., 1.);
        } else {
            audio.play(asset_library.audio("waterdrop"));
            commands.entity(entity).despawn();
        }
    }
}

pub fn hit_boats(
    mut end_game: ResMut<EndGame>,
    query: Query<(Entity, &EgCannonBall, &Transform, &Collision)>,
    mut boat_query: Query<&mut EgBoat>,
    collision_query: Res<CollisionQuery>,
    audio: Res<Audio>,
    asset_library: Res<AssetLibrary>,
) {
    for (entity, cannon_ball, transform, collision) in query.iter() {
        if cannon_ball.landing_size > 0.35 {
            let filter = Some(CollisionFilter {
                exclude_entity: entity,
                flags: 0x1000,
            });
            if let Some(response) =
                collision_query.check(transform.translation.truncate(), collision.shape, filter)
            {
                if let Ok(mut boat) = boat_query.get_mut(response.entity) {
                    if boat.hit() {
                        if end_game.your_health != 0 {
                            end_game.your_health -= 1;
                        }
                        audio.play(asset_library.audio("boathit"));
                    }
                }
            }
        }
    }
}

pub fn spawn_cannons(mut commands: Commands, asset_library: Res<AssetLibrary>, audio: Res<Audio>) {
    let spawn_chance = 0.07;
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
            .insert(EgCannonBall {
                falling_size: 0.0,
                landing_size: 0.0,
                speed: 0.1,
                dir: Vec2::ZERO,
                dir_set: false,
            })
            .insert(EndGameEntity);
    }
}
