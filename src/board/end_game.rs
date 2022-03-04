use crate::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::Audio;

pub enum EndGameState {
    Win,
    Win2,
    Lose,
    Tie,
}

pub struct EndGame {
    state: EndGameState,
}

pub struct EndGamePlugin;

impl Plugin for EndGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EndGame {
            state: EndGameState::Lose,
        })
        .add_system_set(SystemSet::on_enter(BoardState::EndGame).with_system(init))
        .add_system_set(SystemSet::on_update(BoardState::EndGame).with_system(update));
    }
}

pub fn init(
    mut end_game: ResMut<EndGame>,
    game: Res<Game>,
    board: Res<Board>,
    mut dialogue: ResMut<Dialogue>,
    mut camera_controller: ResMut<CameraController>,
) {
    if game.my_coins == 0 && game.your_coins == 0 {
        end_game.state = EndGameState::Tie;
    } else if game.your_coins == 0 {
        end_game.state = EndGameState::Lose;
    } else if game.my_coins == 0 {
        end_game.state = EndGameState::Win;
    }
    match end_game.state {
        EndGameState::Tie => {
            camera_controller.follow_entity = board.your_pawn;
            dialogue.add(DialogueEntry {
                text: "Oh.. it looks like you ran out of coins.".into(),
                ..Default::default()
            });
            dialogue.add(DialogueEntry {
                text: "So did I.. but this is my game, my rules.".into(),
                ..Default::default()
            });
            dialogue.add(DialogueEntry {
                text: "You lose! Sorry!".into(),
                ..Default::default()
            });
            dialogue.add(DialogueEntry {
                text: "Better luck next time.".into(),
                ..Default::default()
            });
        }
        EndGameState::Win => {
            camera_controller.follow_entity = board.my_pawn;
            dialogue.add(DialogueEntry {
                text: "...".into(),
                ..Default::default()
            });
            dialogue.add(DialogueEntry {
                text: "Very impressive.".into(),
                ..Default::default()
            });
            dialogue.add(DialogueEntry {
                text: "But did you think I would lose so easily?".into(),
                ..Default::default()
            });
        }
        EndGameState::Win2 => {}
        EndGameState::Lose => {
            camera_controller.follow_entity = board.your_pawn;
            dialogue.add(DialogueEntry {
                text: "Oh.. it looks like you ran out of coins.".into(),
                ..Default::default()
            });
            dialogue.add(DialogueEntry {
                text: "Better luck next time.".into(),
                ..Default::default()
            });
        }
    }
}

pub fn update(
    mut end_game: ResMut<EndGame>,
    mut dialogue: ResMut<Dialogue>,
    mut reset: EventWriter<GameResetSend>,
    mut board: ResMut<Board>,
    mut game_state: ResMut<State<GameState>>,
    time: Res<Time>,
    audio: Res<Audio>,
    asset_library: Res<AssetLibrary>,
) {
    if !dialogue.busy() {
        match end_game.state {
            EndGameState::Tie => {
                reset.send(GameResetSend);
            }
            EndGameState::Win => {
                if board.my_item_use_interpolate == 0. {
                    audio.play(asset_library.audio("itemuse"));
                }
                board.my_item_use_interpolate += time.delta_seconds() * 0.75;
                board.my_item_use_interpolate = board.my_item_use_interpolate.clamp(0., 1.);
                if board.my_item_use_interpolate >= 1. {
                    dialogue.add(DialogueEntry {
                        text: "I use my trump card!".into(),
                        ..Default::default()
                    });
                    dialogue.add(DialogueEntry {
                        text: "Prepare yourself!".into(),
                        ..Default::default()
                    });
                    end_game.state = EndGameState::Win2;
                }
            }
            EndGameState::Win2 => {
                game_state.set(GameState::EndGame).unwrap();
            }
            EndGameState::Lose => {
                reset.send(GameResetSend);
            }
        }
    }
}
