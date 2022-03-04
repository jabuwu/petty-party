use super::pawn::Pawn;
use crate::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;

pub struct Moving {
    sent_dialogue: bool,
}

pub struct MovingPlugin;

impl Plugin for MovingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Moving {
            sent_dialogue: false,
        })
        .add_system_set(SystemSet::on_enter(BoardState::Moving).with_system(init))
        .add_system_set(SystemSet::on_update(BoardState::Moving).with_system(update));
    }
}

pub fn init(
    game: Res<Game>,
    mut board: ResMut<Board>,
    mut moving: ResMut<Moving>,
    mut dice_start: EventWriter<DiceRollStart>,
    difficulty: Res<Difficulty>,
) {
    moving.sent_dialogue = false;
    board.moving = true;
    let mut rng = rand::thread_rng();
    if board.my_turn {
        if game.turn == 1 {
            board.moves = 5;
        } else {
            board.moves = if game.turn % 2 == 1 { 4 } else { 5 };
        }
    } else {
        match *difficulty {
            Difficulty::Normal => {
                board.moves = 6;
            }
            Difficulty::Hard => {
                board.moves = rng.gen_range(1..=2) * 3;
            }
        }
    }
    dice_start.send(DiceRollStart { value: board.moves });
}

pub fn update(
    mut game: ResMut<Game>,
    mut board: ResMut<Board>,
    mut board_state: ResMut<State<BoardState>>,
    mut dialogue: ResMut<Dialogue>,
    pawn_query: Query<&Pawn>,
    mut moving: ResMut<Moving>,
    difficulty: Res<Difficulty>,
) {
    if !board.moving && !moving.sent_dialogue && !board.my_turn {
        if game.turn == 1 {
            if matches!(*difficulty, Difficulty::Normal) {
                dialogue.add(DialogueEntry {
                    text: "Oh, bad luck landing on red. You lost a coin!".into(),
                    ..Default::default()
                });
            } else {
                dialogue.add(DialogueEntry {
                    text: "Oh, bad luck landing on red. You lose 3 coins!".into(),
                    ..Default::default()
                });
            }
            dialogue.add(DialogueEntry {
                text: "If you run out of coins, you lose the game!".into(),
                ..Default::default()
            });
        } else if game.turn == 2 {
            dialogue.add(DialogueEntry {
                text: "Another red! Too Bad!".into(),
                ..Default::default()
            });
        } else if game.turn == 3 {
            dialogue.add(DialogueEntry {
                text: "Red again?".into(),
                ..Default::default()
            });
            dialogue.add(DialogueEntry {
                text: "It's almost like the game is rigged...!".into(),
                ..Default::default()
            });
        }
        moving.sent_dialogue = true;
    }
    if !board.moving && !dialogue.busy() {
        if board.my_turn {
            if let Some(my_pawn) = board.my_pawn {
                if let Ok(pawn) = pawn_query.get(my_pawn) {
                    match pawn.tile_type {
                        TileType::Blue => {
                            if matches!(*difficulty, Difficulty::Normal) {
                                game.my_coins += 1;
                            } else {
                                game.my_coins += 3;
                            }
                        }
                        TileType::Red => {
                            if matches!(*difficulty, Difficulty::Normal) {
                                if game.my_coins <= 1 {
                                    game.my_coins = 0;
                                } else {
                                    game.my_coins -= 1;
                                }
                            } else {
                                if game.my_coins <= 3 {
                                    game.my_coins = 0;
                                } else {
                                    game.my_coins -= 3;
                                }
                            }
                        }
                        TileType::Green => {}
                    }
                }
            }
            board.my_turn = false;
            board_state.set(BoardState::TurnIntro).unwrap();
        } else {
            if let Some(your_pawn) = board.your_pawn {
                if let Ok(pawn) = pawn_query.get(your_pawn) {
                    match pawn.tile_type {
                        TileType::Blue => {
                            if matches!(*difficulty, Difficulty::Normal) {
                                game.your_coins += 1;
                            } else {
                                game.your_coins += 3;
                            }
                        }
                        TileType::Red => {
                            if matches!(*difficulty, Difficulty::Normal) {
                                if game.your_coins <= 1 {
                                    game.your_coins = 0;
                                } else {
                                    game.your_coins -= 1;
                                }
                            } else {
                                if game.your_coins <= 3 {
                                    game.your_coins = 0;
                                } else {
                                    game.your_coins -= 3;
                                }
                            }
                        }
                        TileType::Green => {}
                    }
                }
            }
            board_state.set(BoardState::Ending).unwrap();
        }
    }
}
