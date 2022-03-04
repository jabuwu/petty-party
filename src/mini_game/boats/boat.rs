use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Boat {
    pub movement: Vec2,
    pub invulnerable_timer: f32,
    pub coins: u32,
    pub my_boat: bool,
}

impl Boat {
    pub fn hit(&mut self) -> bool {
        if self.coins > 0 && self.invulnerable_timer == 0. {
            self.invulnerable_timer = 1.;
            if self.coins < 2 {
                self.coins = 0;
            } else {
                self.coins -= 2;
            }
            true
        } else {
            false
        }
    }
}

pub struct BoatPlugin;

impl Plugin for BoatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(MiniGameState::Boats).with_system(update));
    }
}

pub fn update(
    game: Res<Game>,
    mut boat_query: Query<(Entity, &mut Transform, &Collision, &mut Boat, &mut Sprite)>,
    collision_query: Res<CollisionQuery>,
    timer: Res<Time>,
    mini_game: Res<MiniGame>,
) {
    for (_, _, _, boat, mut sprite) in boat_query.iter_mut() {
        let color = if boat.my_boat {
            game.my_color
        } else {
            game.your_color
        };
        sprite.color = color;
    }
    if !mini_game.active {
        return;
    }
    for (entity, mut transform, collision, mut boat, mut sprite) in boat_query.iter_mut() {
        let movement = boat.movement;
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
        let mut color = if boat.my_boat {
            game.my_color
        } else {
            game.your_color
        };
        if boat.invulnerable_timer > 0. {
            color.set_a(0.2);
        }
        sprite.color = color;
    }
}
