use crate::prelude::*;
use bevy::prelude::*;

pub struct EndingPlugin;

impl Plugin for EndingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Ending).with_system(enter))
            .add_system_set(SystemSet::on_update(GameState::Ending).with_system(update));
    }
}

pub fn enter(mut dialogue: ResMut<Dialogue>) {
    dialogue.add(DialogueEntry {
        text: "Thank you for playing.".into(),
        color: Color::WHITE,
        ..Default::default()
    });
    dialogue.add(DialogueEntry {
        text: "This was a game made for Bevy Jam #1.".into(),
        color: Color::WHITE,
        ..Default::default()
    });
    dialogue.add(DialogueEntry {
        text: "I had to nerf the difficulty of the game a lot. I found it was too\ndifficult for a game jam submission. If you want to try the\noriginal difficulty, press 1 at the color select screen."
            .into(),
        color: Color::WHITE,
        ..Default::default()
    });
}

pub fn update(game: Res<Game>, dialogue: Res<Dialogue>, mut reset: EventWriter<GameResetSend>) {
    if dialogue.busy() || game.dice_roll {
        return;
    }
    reset.send(GameResetSend);
}
