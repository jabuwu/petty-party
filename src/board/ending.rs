use crate::prelude::*;
use bevy::prelude::*;

#[derive(Default)]
pub struct Ending {
    animation: Option<Animation>,
}

pub struct EndingPlugin;

impl Plugin for EndingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Ending { animation: None })
            .add_system_set(SystemSet::on_enter(BoardState::Ending).with_system(init))
            .add_system_set(SystemSet::on_exit(BoardState::Ending).with_system(cleanup))
            .add_system_set(SystemSet::on_update(BoardState::Ending).with_system(update));
    }
}

pub fn init(
    game: Res<Game>,
    mut ending: ResMut<Ending>,
    mut camera_controller: ResMut<CameraController>,
    mut dialogue: ResMut<Dialogue>,
) {
    ending.animation = Some(Animation::stub());
    camera_controller.follow_entity = None;
    if game.turn == 1 {
        dialogue.add(DialogueEntry {
            text: "Now we play a mini game!".into(),
            ..Default::default()
        });
        dialogue.add(DialogueEntry {
            text: "You can practice as many times as you like!".into(),
            ..Default::default()
        });
        dialogue.add(DialogueEntry {
            text: "You'll need it.".into(),
            ..Default::default()
        });
    }
}

pub fn cleanup(mut ending: ResMut<Ending>) {
    ending.animation = None;
}

pub fn update(
    game: Res<Game>,
    mut ending: ResMut<Ending>,
    time: Res<Time>,
    mut game_state: ResMut<State<GameState>>,
    mut board_state: ResMut<State<BoardState>>,
    dialogue: Res<Dialogue>,
) {
    if dialogue.busy() {
        return;
    }
    if game.my_coins == 0 || game.your_coins == 0 {
        board_state.set(BoardState::EndGame).unwrap();
        return;
    }
    if let Some(animation) = &mut ending.animation {
        animation.update(time.delta_seconds());
        if animation.finished() {
            game_state.set(GameState::InfoScreen).unwrap();
        }
    }
}
