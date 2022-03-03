use crate::prelude::*;
use bevy::prelude::*;

pub enum EndGameState {
    Win,
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

pub fn init(mut end_game: ResMut<EndGame>, game: Res<Game>, mut dialogue: ResMut<Dialogue>) {
    if game.my_coins == 0 && game.your_coins == 0 {
        end_game.state = EndGameState::Tie;
    } else if game.your_coins == 0 {
        end_game.state = EndGameState::Lose;
    } else if game.my_coins == 0 {
        end_game.state = EndGameState::Win;
    }
    match end_game.state {
        EndGameState::Tie => {
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
            dialogue.add(DialogueEntry {
                text: "You win. TODO: end game".into(),
                ..Default::default()
            });
        }
        EndGameState::Lose => {
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
    end_game: Res<EndGame>,
    dialogue: Res<Dialogue>,
    mut reset: EventWriter<GameResetSend>,
) {
    if !dialogue.busy() {
        match end_game.state {
            EndGameState::Tie => {
                reset.send(GameResetSend);
            }
            EndGameState::Win => {
                reset.send(GameResetSend);
            }
            EndGameState::Lose => {
                reset.send(GameResetSend);
            }
        }
    }
}
