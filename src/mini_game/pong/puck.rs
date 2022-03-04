use super::Pong;
use crate::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use rand::prelude::*;

#[derive(Component)]
pub struct Puck {
    pub velocity: Vec2,
}

impl Default for Puck {
    fn default() -> Self {
        Self {
            velocity: Vec2::new(0., 0.),
        }
    }
}

pub struct PuckPlugin;

impl Plugin for PuckPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update);
    }
}

pub fn update(
    mut query: Query<(Entity, &mut Transform, &Collision, &mut Puck)>,
    collision_query: Res<CollisionQuery>,
    time: Res<Time>,
    mini_game: ResMut<MiniGame>,
    mut commands: Commands,
    mut pong: ResMut<Pong>,
    audio: Res<Audio>,
    asset_library: Res<AssetLibrary>,
) {
    if !mini_game.active {
        return;
    }
    let mut rng = rand::thread_rng();
    for (entity, mut transform, collision, mut puck) in query.iter_mut() {
        let filter = Some(CollisionFilter {
            exclude_entity: entity,
            flags: 0x1000,
        });
        if puck.velocity.length_squared() < 0.01 {
            let mut angle = rng.gen_range(30.0..60.0f32).to_radians();
            angle += ((rng.gen_range(0..=3u32) * 90) as f32).to_radians();
            puck.velocity = Vec2::new(angle.cos(), angle.sin()) * 60.;
        }
        transform.translation += puck.velocity.extend(0.) * time.delta_seconds();
        if transform.translation.x > 155. {
            puck.velocity.x = puck.velocity.x.abs() * -1.;
        }
        if transform.translation.x < -155. {
            puck.velocity.x = puck.velocity.x.abs();
        }
        if transform.translation.y > 100. {
            pong.your_coins += 2;
            commands.entity(entity).despawn();
        }
        if transform.translation.y < -100. {
            pong.my_coins += 2;
            commands.entity(entity).despawn();
        }
        if let Some(result) =
            collision_query.check(transform.translation.truncate(), collision.shape, filter)
        {
            audio.play(asset_library.audio("pong"));
            let magnitude = puck.velocity.length() + 20.;
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
