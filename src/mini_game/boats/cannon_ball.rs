use crate::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use rand::prelude::*;

use super::boat::Boat;

const ACCELERATION: f32 = 50.0;

#[derive(Component)]
pub struct CannonBall {
    pub falling_size: f32,
    pub landing_size: f32,
    pub speed: f32,
    pub dir: Vec2,
    pub dir_set: bool,
}

pub struct CannonBallPlugin;

impl Plugin for CannonBallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update).add_system(hit_boats);
    }
}

pub fn update(
    mut commands: Commands,
    mut query: Query<(Entity, &mut CannonBall, &mut Transform, &mut Sprite)>,
    time: Res<Time>,
    audio: Res<Audio>,
    asset_library: Res<AssetLibrary>,
) {
    let mut rng = rand::thread_rng();
    for (entity, mut cannon_ball, mut transform, mut sprite) in query.iter_mut() {
        if !cannon_ball.dir_set {
            let angle = Vec2::angle_between(Vec2::new(-1., 0.), transform.translation.truncate())
                + rng.gen_range(-45.0..45.0f32).to_radians();
            let speed = rng.gen_range(10.0..70.0f32);
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
    query: Query<(Entity, &CannonBall, &Transform, &Collision)>,
    mut boat_query: Query<&mut Boat>,
    collision_query: Res<CollisionQuery>,
    mini_game: Res<MiniGame>,
    audio: Res<Audio>,
    asset_library: Res<AssetLibrary>,
) {
    if !mini_game.active {
        return;
    }
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
                        audio.play(asset_library.audio("boathit"));
                    }
                }
            }
        }
    }
}
