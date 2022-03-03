use super::boat::Boat;
use crate::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Default, Component)]
pub struct EnemyBoat {
    angle: f32,
}

pub struct EnemyBoatPlugin;

impl Plugin for EnemyBoatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(MiniGameState::Boats).with_system(update));
    }
}

pub fn update(mut boat_query: Query<(&mut Boat, &mut EnemyBoat)>) {
    let mut rng = rand::thread_rng();
    for (mut boat, mut enemy_boat) in boat_query.iter_mut() {
        if enemy_boat.angle == 0. || rng.gen_bool(0.1) {
            enemy_boat.angle = rng.gen_range(0.0..360.0f32).to_radians();
        }
        boat.movement = Vec2::new(enemy_boat.angle.cos() * 2., enemy_boat.angle.sin() * 2.);
    }
}
