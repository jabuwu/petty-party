use crate::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use std::collections::VecDeque;

#[derive(Default)]
pub struct Dialogue {
    active: bool,
    entries: VecDeque<DialogueEntry>,
}

#[derive(Component)]
pub struct DialogueText;

#[derive(Component)]
pub struct DialogueBg;

impl Dialogue {
    pub fn add(&mut self, entry: DialogueEntry) {
        self.entries.push_back(entry);
        self.active = true;
    }

    pub fn busy(&self) -> bool {
        self.active
    }
}

#[derive(Default)]
pub struct DialogueEntry {
    pub text: String,
}

pub struct DialoguePlugin;

impl Plugin for DialoguePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Dialogue::default())
            .add_system(init)
            .add_system(update)
            .add_system(reset);
    }
}

pub fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_library_ready: EventReader<AssetLibraryReady>,
) {
    for _ in asset_library_ready.iter() {
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(40.0)),
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        bottom: Val::Px(0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                color: Color::rgba(0.1, 0.1, 0.1, 0.97).into(),
                ..Default::default()
            })
            .insert(DialogueBg)
            .with_children(|parent| {
                parent
                    .spawn_bundle(TextBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            position: Rect {
                                top: Val::Px(20.),
                                left: Val::Px(20.),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        text: Text::with_section(
                            "",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 24.0,
                                color: Color::rgba(1., 0.7, 0.7, 1.0),
                            },
                            TextAlignment {
                                horizontal: HorizontalAlign::Center,
                                vertical: VerticalAlign::Center,
                            },
                        ),
                        ..Default::default()
                    })
                    .insert(DialogueText);
            });
    }
}

#[derive(Default)]
pub struct AudioState {
    played: bool,
}

pub fn update(
    mut dialogue: ResMut<Dialogue>,
    input: Res<Input<KeyCode>>,
    mut text_query: Query<&mut Text, With<DialogueText>>,
    mut bg_query: Query<&mut Visibility, With<DialogueBg>>,
    mut audio_state: Local<AudioState>,
    audio: Res<Audio>,
    asset_library: Res<AssetLibrary>,
) {
    if !dialogue.entries.is_empty() {
        if !audio_state.played {
            audio.play(asset_library.audio("dialogue"));
            audio_state.played = true;
        }
        for mut text in text_query.iter_mut() {
            if text.sections[0].value != dialogue.entries[0].text {
                text.sections[0].value = dialogue.entries[0].text.clone();
            }
        }
        for mut bg in bg_query.iter_mut() {
            bg.is_visible = true;
        }
        dialogue.active = true;
        if input.just_pressed(KeyCode::Space) {
            dialogue.entries.pop_front();
            audio_state.played = false;
        }
    } else {
        audio_state.played = false;
        for mut text in text_query.iter_mut() {
            text.sections[0].value = "".into();
        }
        for mut bg in bg_query.iter_mut() {
            bg.is_visible = false;
        }
        dialogue.active = false;
    }
}

pub fn reset(mut reset: EventReader<GameReset>, mut dialogue: ResMut<Dialogue>) {
    for _ in reset.iter() {
        dialogue.entries.clear();
        dialogue.active = false;
    }
}
