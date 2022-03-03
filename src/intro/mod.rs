use crate::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::Audio;

pub enum IntroState {
    Dialogue1,
    DiceRoll1,
    Dialogue2,
    DiceRoll2,
    Dialogue3,
    End,
}

pub struct IntroPlugin;

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(IntroState::Dialogue1)
            .add_system_set(SystemSet::on_enter(GameState::Intro).with_system(enter))
            .add_system_set(SystemSet::on_update(GameState::Intro).with_system(update));
    }
}

pub fn enter(mut intro: ResMut<IntroState>, audio: Res<Audio>, asset_library: Res<AssetLibrary>) {
    //audio.play_looped(asset_library.audio("music"));
    *intro = IntroState::Dialogue1;
}

pub fn update(
    mut game: ResMut<Game>,
    mut intro: ResMut<IntroState>,
    mut dialogue: ResMut<Dialogue>,
    mut game_state: ResMut<State<GameState>>,
    mut dice_start: EventWriter<DiceRollStart>,
    mut dice_hide: EventWriter<DiceRollHide>,
) {
    if dialogue.busy() || game.dice_roll {
        return;
    }
    match *intro {
        IntroState::Dialogue1 => {
            dialogue.add(DialogueEntry {
                text: "Hi! Welcome!".into(),
                ..Default::default()
            });
            dialogue.add(DialogueEntry {
                text: "Are you here to play my game!?".into(),
                ..Default::default()
            });
            dialogue.add(DialogueEntry {
                text: "No one has beaten it yet...".into(),
                ..Default::default()
            });
            dialogue.add(DialogueEntry {
                text: "They all quit on me...".into(),
                ..Default::default()
            });
            dialogue.add(DialogueEntry {
                text: "Will you be the first to beat my game?".into(),
                ..Default::default()
            });
            dialogue.add(DialogueEntry {
                text: "Oh!! Great!!".into(),
                ..Default::default()
            });
            dialogue.add(DialogueEntry {
                text: "We start by rolling a die to see who goes first.".into(),
                ..Default::default()
            });
            *intro = IntroState::DiceRoll1;
        }
        IntroState::DiceRoll1 => {
            dice_start.send(DiceRollStart { value: 10 });
            game.dice_roll = true;
            *intro = IntroState::Dialogue2;
        }
        IntroState::Dialogue2 => {
            dialogue.add(DialogueEntry {
                text: "Oh, I rolled a 10. I guess I will go first!".into(),
                ..Default::default()
            });
            dialogue.add(DialogueEntry {
                text: "Oh.. right.. you can roll too if you want.".into(),
                ..Default::default()
            });
            *intro = IntroState::DiceRoll2;
        }
        IntroState::DiceRoll2 => {
            dice_start.send(DiceRollStart { value: 3 });
            game.dice_roll = true;
            *intro = IntroState::Dialogue3;
        }
        IntroState::Dialogue3 => {
            dialogue.add(DialogueEntry {
                text: "Too bad.".into(),
                ..Default::default()
            });
            dialogue.add(DialogueEntry {
                text: "Anyway, let's start!".into(),
                ..Default::default()
            });
            *intro = IntroState::End;
        }
        IntroState::End => {
            dice_hide.send(DiceRollHide);
            game_state.set(GameState::Board).unwrap();
        }
    }
}
