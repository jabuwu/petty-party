use crate::prelude::*;
use bevy::prelude::*;
use end_game::EndGamePlugin;
use ending::EndingPlugin;
use free_cam::FreeCamPlugin;
use item::Item;
use moving::MovingPlugin;
use pawn::{Pawn, PawnPlugin};
use score_overlay::ScoreOverlayPlugin;
use shop::ShopPlugin;
use starting::StartingPlugin;
use tile::{tiles, Tile, TileType};
use turn_input::TurnInputPlugin;
use turn_intro::TurnIntroPlugin;
use use_item::UseItemPlugin;

pub struct Board {
    pub my_turn: bool,
    pub score_overlay: bool,
    pub my_pawn: Option<Entity>,
    pub your_pawn: Option<Entity>,
    pub active_pawn: Option<Entity>,
    pub moving: bool,
    pub shop: bool,
    pub first_shop: bool,
    pub moves: u32,
    pub my_item: Item,
    pub your_item: Item,
    pub rapier_dialog: bool,
}

impl Default for Board {
    fn default() -> Self {
        Self {
            my_turn: true,
            score_overlay: false,
            my_pawn: None,
            your_pawn: None,
            active_pawn: None,
            moving: false,
            shop: false,
            first_shop: true,
            moves: 3,
            my_item: Item::TrumpCard,
            your_item: Item::None,
            rapier_dialog: true,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum BoardState {
    Inactive,
    Starting,
    TurnIntro,
    TurnInput,
    UseItem,
    Moving,
    FreeCam,
    Ending,
    EndGame,
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(StartingPlugin)
            .add_plugin(TurnIntroPlugin)
            .add_plugin(TurnInputPlugin)
            .add_plugin(FreeCamPlugin)
            .add_plugin(MovingPlugin)
            .add_plugin(EndingPlugin)
            .add_plugin(ScoreOverlayPlugin)
            .add_plugin(ShopPlugin)
            .add_plugin(PawnPlugin)
            .add_plugin(UseItemPlugin)
            .add_plugin(EndGamePlugin)
            .add_state(BoardState::Inactive)
            .insert_resource(Board::default())
            .add_system_set(SystemSet::on_enter(GameState::Board).with_system(enter))
            .add_system_set(SystemSet::on_exit(GameState::Board).with_system(exit))
            .add_system(init)
            .add_system(reset);
    }
}

pub fn init(
    mut board: ResMut<Board>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    mut asset_library_ready: EventReader<AssetLibraryReady>,
) {
    for _ in asset_library_ready.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_library.image("bg"),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                visibility: Visibility { is_visible: false },
                ..Default::default()
            })
            .insert(SceneVisibility(GameState::Board));
        let tiles = tiles();
        let mut entities = vec![];
        for _ in tiles.iter() {
            entities.push(commands.spawn().id());
        }
        for (i, tile_def) in tiles.iter().enumerate() {
            let next = if i == entities.len() - 1 {
                vec![entities[1]]
            } else {
                vec![entities[i + 1]]
            };
            commands
                .entity(entities[i])
                .insert_bundle(SpriteBundle {
                    texture: asset_library.image(match tile_def.tile_type {
                        TileType::Blue => "tile_blue",
                        TileType::Red => "tile_red",
                        TileType::Green => "tile_green",
                    }),
                    visibility: Visibility { is_visible: false },
                    transform: Transform::from_xyz(tile_def.position.x, tile_def.position.y, 0.1),
                    ..Default::default()
                })
                .insert(Tile {
                    tile_type: tile_def.tile_type,
                    next,
                })
                .insert(SceneVisibility(GameState::Board));
        }
        let my_pawn = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Vec2::new(10.0, 10.0).into(),
                    color: Color::RED,
                    ..Default::default()
                },
                visibility: Visibility { is_visible: false },
                transform: Transform::from_xyz(tiles[0].position.x, tiles[0].position.y, 0.41),
                ..Default::default()
            })
            .insert(Pawn {
                tile: entities[0],
                tile_type: TileType::Blue,
                player: false,
                first_dec: true,
            })
            .insert(SceneVisibility(GameState::Board))
            .id();
        let your_pawn = commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    custom_size: Vec2::new(10.0, 10.0).into(),
                    color: Color::RED,
                    ..Default::default()
                },
                visibility: Visibility { is_visible: false },
                transform: Transform::from_xyz(tiles[0].position.x, tiles[0].position.y, 0.4),
                ..Default::default()
            })
            .insert(Pawn {
                tile: entities[0],
                tile_type: TileType::Blue,
                player: true,
                first_dec: true,
            })
            .insert(SceneVisibility(GameState::Board))
            .id();
        board.my_pawn = Some(my_pawn);
        board.your_pawn = Some(your_pawn);
    }
}

pub fn enter(mut board: ResMut<Board>, mut board_state: ResMut<State<BoardState>>) {
    board.my_turn = true;
    board.score_overlay = true;
    board.active_pawn = board.my_pawn;
    board_state.set(BoardState::Starting).unwrap();
}

pub fn exit(mut board: ResMut<Board>, mut board_state: ResMut<State<BoardState>>) {
    board.score_overlay = false;
    board_state.set(BoardState::Inactive).unwrap();
}

pub fn reset(mut reset: EventReader<GameReset>, mut board: ResMut<Board>) {
    for _ in reset.iter() {
        *board = Board::default();
    }
}

mod end_game;
mod ending;
mod free_cam;
mod item;
mod moving;
mod pawn;
mod score_overlay;
mod shop;
mod starting;
mod tile;
mod turn_input;
mod turn_intro;
mod use_item;

pub mod prelude {
    pub use super::{
        item::Item,
        shop::ShopOpen,
        tile::{Tile, TileType},
        Board, BoardState,
    };
}
