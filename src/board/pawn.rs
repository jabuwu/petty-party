use crate::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use std::collections::HashMap;

#[derive(Component)]
pub struct Pawn {
    pub tile: Entity,
    pub tile_type: TileType,
    pub player: bool,
    pub first_dec: bool,
}

pub struct PawnPlugin;

impl Plugin for PawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(pawn_move).add_system(pawn_color);
    }
}

pub fn pawn_move(
    game: Res<Game>,
    mut board: ResMut<Board>,
    mut queries: QuerySet<(
        QueryState<(Entity, &Transform, &Tile)>,
        QueryState<(Entity, &mut Pawn, &mut Transform)>,
    )>,
    time: Res<Time>,
    mut shop_open: EventWriter<ShopOpen>,
    mut dice_value: EventWriter<DiceRollValue>,
    mut dice_hide: EventWriter<DiceRollHide>,
    audio: Res<Audio>,
    asset_library: Res<AssetLibrary>,
) {
    let tile_info: HashMap<Entity, (Vec2, Vec<Entity>, TileType)> = queries
        .q0()
        .iter()
        .map(|(e, t, bt)| (e, (t.translation.truncate(), bt.next.clone(), bt.tile_type)))
        .collect();
    for (entity, mut pawn, mut pawn_transform) in queries.q1().iter_mut() {
        let is_active = if let Some(active_pawn) = board.active_pawn {
            active_pawn == entity
        } else {
            false
        };
        if board.moving && !game.dice_roll && !board.shop && is_active {
            if let Some((target_position, next_tiles, tile_type)) = tile_info.get(&pawn.tile) {
                let mut position = pawn_transform.translation.truncate();
                let difference = *target_position - position;
                let magnitude = difference.length();
                let speed = time.delta_seconds() * 200.;
                if magnitude < speed {
                    if matches!(*tile_type, TileType::Green) && !board.my_turn {
                        board.shop = true;
                        shop_open.send(ShopOpen);
                    }
                    if board.moves > 0 {
                        audio.play(asset_library.audio("move"));
                        dice_value.send(DiceRollValue { value: board.moves });
                        board.moves -= 1;
                        position = *target_position;
                        pawn.tile = next_tiles[0];
                    } else {
                        audio.play(asset_library.audio("move"));
                        dice_hide.send(DiceRollHide);
                        pawn.tile_type = *tile_type;
                        board.moving = false;
                    }
                } else {
                    let movement = difference.normalize_or_zero() * speed;
                    position += movement;
                }
                pawn_transform.translation.x = position.x;
                pawn_transform.translation.y = position.y;
            }
        } else {
            pawn.first_dec = true;
        }
    }
}

pub fn pawn_color(game: Res<Game>, mut query: Query<(&mut Sprite, &Pawn)>) {
    for (mut sprite, pawn) in query.iter_mut() {
        if pawn.player {
            sprite.color = game.your_color;
        } else {
            sprite.color = game.my_color;
        }
    }
}
