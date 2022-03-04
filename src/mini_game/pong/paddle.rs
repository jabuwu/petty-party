use super::puck::Puck;
use crate::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct PaddlePlayer;

#[derive(Component, Default)]
pub struct PaddleAi {
    target: f32,
    offset: f32,
}

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(MiniGameState::Pong).with_system(update));
        app.add_system_set(SystemSet::on_update(MiniGameState::Pong).with_system(update_ai_target));
    }
}

pub fn update(
    mut paddle_query: Query<
        (&mut Transform, Option<&PaddlePlayer>, Option<&PaddleAi>),
        With<Paddle>,
    >,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mini_game: Res<MiniGame>,
) {
    if !mini_game.active {
        return;
    }
    for (mut transform, player, ai) in paddle_query.iter_mut() {
        if player.is_some() {
            if input.pressed(KeyCode::A) {
                transform.translation.x -= time.delta_seconds() * 250.;
            }
            if input.pressed(KeyCode::D) {
                transform.translation.x += time.delta_seconds() * 250.;
            }
        }
        if let Some(ai) = ai {
            let dist = (ai.target - transform.translation.x).abs();
            if dist > 20. {
                if ai.target > transform.translation.x {
                    transform.translation.x += time.delta_seconds() * 150.;
                    if transform.translation.x > ai.target {
                        transform.translation.x = ai.target;
                    }
                } else if ai.target < transform.translation.x {
                    transform.translation.x -= time.delta_seconds() * 150.;
                    if transform.translation.x < ai.target {
                        transform.translation.x = ai.target;
                    }
                }
            }
        }
        transform.translation.x = transform.translation.x.clamp(-140., 140.);
    }
}

pub fn update_ai_target(
    mut query: QuerySet<(
        QueryState<(Entity, &Transform), With<Puck>>,
        QueryState<(&Transform, &mut PaddleAi)>,
    )>,
    mini_game: Res<MiniGame>,
) {
    if !mini_game.active {
        return;
    }
    let mut rng = rand::thread_rng();
    let mut pucks: Vec<(Entity, Vec2)> = query
        .q0()
        .iter()
        .map(|(e, t)| (e, t.translation.truncate()))
        .collect();
    for (transform, mut ai) in query.q1().iter_mut() {
        let mut closest: Option<(usize, Entity, f32, f32)> = None;
        for (i, (puck_entity, puck_position)) in pucks.iter().enumerate() {
            let distance = (transform.translation.truncate() - *puck_position).length();
            if let Some(closest_some) = closest {
                if distance < closest_some.2 {
                    closest = Some((i, *puck_entity, distance, puck_position.x));
                }
            } else {
                closest = Some((i, *puck_entity, distance, puck_position.x));
            }
        }
        if let Some(closest) = closest {
            pucks.remove(closest.0);
            if rng.gen_bool(0.05) {
                ai.offset = rng.gen_range(-10.0..10.0f32);
            }
            ai.target = closest.3 + ai.offset;
        } else {
            if rng.gen_bool(0.05) {
                ai.offset = rng.gen_range(-100.0..100.0f32);
            }
            ai.target = ai.offset;
        }
    }
}
