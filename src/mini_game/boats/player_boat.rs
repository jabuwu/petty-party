use super::boat::Boat;
use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerBoat;

pub struct PlayerBoatPlugin;

impl Plugin for PlayerBoatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(MiniGameState::Boats).with_system(update));
    }
}

pub fn update(mut boat_query: Query<&mut Boat, With<PlayerBoat>>, input: Res<Input<KeyCode>>) {
    for mut boat in boat_query.iter_mut() {
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
    }
}
