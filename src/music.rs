use crate::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel};

pub struct MusicPlugin;

#[derive(Debug, PartialEq, Eq)]
pub enum Music {
    None,
    Board,
    Info,
    MiniGame,
    EndGame,
}

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(play_music);
    }
}

pub struct MusicState {
    channel: AudioChannel,
    last_state: GameState,
    last_music: Music,
}

impl Default for MusicState {
    fn default() -> Self {
        Self {
            channel: AudioChannel::new("music".into()),
            last_state: GameState::Loading,
            last_music: Music::None,
        }
    }
}

pub fn play_music(
    game_state: Res<State<GameState>>,
    mut state: Local<MusicState>,
    asset_library: Res<AssetLibrary>,
    audio: Res<Audio>,
) {
    if state.last_state != *game_state.current() {
        state.last_state = *game_state.current();
        let desired_music = match *game_state.current() {
            GameState::Intro => Music::Board,
            GameState::Board => Music::Board,
            GameState::InfoScreen => Music::Info,
            GameState::EndGame => Music::EndGame,
            GameState::MiniGame => Music::MiniGame,
            _ => Music::None,
        };
        if desired_music != state.last_music {
            audio.stop_channel(&state.channel);
            match desired_music {
                Music::Board => {
                    audio.play_looped_in_channel(asset_library.audio("m_board"), &state.channel);
                }
                Music::Info => {
                    audio.play_looped_in_channel(asset_library.audio("m_info"), &state.channel);
                }
                Music::MiniGame => {
                    audio.play_looped_in_channel(asset_library.audio("m_mini"), &state.channel);
                }
                Music::EndGame => {
                    audio.play_looped_in_channel(asset_library.audio("m_endgame"), &state.channel);
                }
                _ => {}
            }
            audio.set_volume_in_channel(0.8, &state.channel);
            state.last_music = desired_music;
        }
    }
}
