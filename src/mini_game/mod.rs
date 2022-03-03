use crate::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::Audio;
use boats::BoatsPlugin;
use duel::DuelPlugin;
use rps::RpsPlugin;

pub struct MiniGame {
    pub active: bool,
    pub start: bool,
    pub finish: bool,
    pub practice: bool,
    pub display_my_coins: u32,
    pub display_your_coins: u32,
}

#[derive(Component)]
pub struct MiniGameEntity;

pub struct MiniGameFinish {
    my_coins: i32,
    your_coins: i32,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum MiniGameState {
    Inactive,
    Rps,
    Boats,
    Duel,
}

#[derive(Component)]
pub struct MiniGameCoinsDisplay {
    mine: bool,
}

#[derive(Component)]
pub struct ReadyText {
    ready_time: f32,
    start_time: f32,
    finish_time: f32,
}

#[derive(Component)]
pub struct PracticeText;

pub struct MiniGamePlugin;

impl Plugin for MiniGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(MiniGameState::Inactive)
            .insert_resource(MiniGame {
                active: false,
                start: false,
                finish: false,
                practice: false,
                display_my_coins: 0,
                display_your_coins: 0,
            })
            .add_event::<MiniGameFinish>()
            .add_plugin(RpsPlugin)
            .add_plugin(BoatsPlugin)
            .add_plugin(DuelPlugin)
            .add_system_set(SystemSet::on_enter(GameState::MiniGame).with_system(enter))
            .add_system_set(SystemSet::on_exit(GameState::MiniGame).with_system(exit))
            .add_system_set(SystemSet::on_update(GameState::MiniGame).with_system(update))
            .add_system_set(SystemSet::on_update(GameState::MiniGame).with_system(practice_text))
            .add_system_set(SystemSet::on_update(GameState::MiniGame).with_system(coin_text));
    }
}

pub fn enter(
    game: Res<Game>,
    mut commands: Commands,
    mut camera_controller: ResMut<CameraController>,
    mut mini_game: ResMut<MiniGame>,
    asset_server: Res<AssetServer>,
    mut dialogue: ResMut<Dialogue>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 42.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            vertical: VerticalAlign::Center,
                        },
                    ),
                    ..Default::default()
                })
                .insert(ReadyText {
                    ready_time: 0.0,
                    start_time: 0.0,
                    finish_time: 0.0,
                });
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: Rect {
                            bottom: Val::Px(20.0),
                            left: Val::Auto,
                            right: Val::Auto,
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "Press ENTER to quit practice",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 24.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            vertical: VerticalAlign::Center,
                        },
                    ),
                    ..Default::default()
                })
                .insert(PracticeText);
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
                        "Coins: 0",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 48.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            vertical: VerticalAlign::Center,
                        },
                    ),
                    ..Default::default()
                })
                .insert(MiniGameCoinsDisplay { mine: false });
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: Rect {
                            top: Val::Px(20.),
                            right: Val::Px(20.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "Coins: 0",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 48.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            vertical: VerticalAlign::Center,
                        },
                    ),
                    ..Default::default()
                })
                .insert(MiniGameCoinsDisplay { mine: true });
        })
        .insert(MiniGameEntity);
    camera_controller.center = true;
    mini_game.start = true;
    mini_game.finish = false;
    mini_game.active = false;
    if game.turn == 1 && !mini_game.practice {
        dialogue.add(DialogueEntry {
            text: "Huh? How did my boat get smaller?".into(),
            ..Default::default()
        });
        dialogue.add(DialogueEntry {
            text: "Must be a bug...".into(),
            ..Default::default()
        });
        dialogue.add(DialogueEntry {
            text: "Well, never mind that! Let's start!".into(),
            ..Default::default()
        });
    } else if game.turn == 2 && !mini_game.practice {
        dialogue.add(DialogueEntry {
            text: "Good ol rock paper scissors!".into(),
            ..Default::default()
        });
        dialogue.add(DialogueEntry {
            text: "Nothing fishy going on here.".into(),
            ..Default::default()
        });
    }
}

pub fn exit(
    mut mini_game: ResMut<MiniGame>,
    mut mini_game_state: ResMut<State<MiniGameState>>,
    destroy_query: Query<Entity, With<MiniGameEntity>>,
    mut commands: Commands,
) {
    if !matches!(mini_game_state.current(), MiniGameState::Inactive) {
        mini_game.practice = false;
        mini_game_state.set(MiniGameState::Inactive).unwrap();
        for entity in destroy_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn update(
    mut game: ResMut<Game>,
    mut mini_game_finish: EventReader<MiniGameFinish>,
    mut game_state: ResMut<State<GameState>>,
    mut mini_game: ResMut<MiniGame>,
    mut text_query: Query<(&mut Text, &mut ReadyText)>,
    mut input: ResMut<Input<KeyCode>>,
    timer: Res<Time>,
    dialogue: Res<Dialogue>,
    audio: Res<Audio>,
    asset_library: Res<AssetLibrary>,
) {
    if dialogue.busy() {
        for (mut text, _) in text_query.iter_mut() {
            text.sections[0].value = "".into();
        }
        return;
    }
    if matches!(game_state.current(), GameState::MiniGame) && mini_game.active {
        let mut done = false;
        for event in mini_game_finish.iter() {
            if !done {
                if !mini_game.practice {
                    if event.my_coins < 0 {
                        if event.my_coins > game.my_coins as i32 {
                            game.my_coins = 0;
                        } else {
                            game.my_coins += event.my_coins as u32;
                        }
                    } else {
                        game.my_coins += event.my_coins as u32;
                    }
                    if event.your_coins < 0 {
                        if event.your_coins > game.your_coins as i32 {
                            game.your_coins = 0;
                        } else {
                            game.your_coins += event.your_coins as u32;
                        }
                    } else {
                        game.your_coins += event.your_coins as u32;
                    }
                }
                mini_game.active = false;
                mini_game.finish = true;
                done = true;
            }
        }
    }
    if input.just_pressed(KeyCode::Return)
        && mini_game.practice
        && (mini_game.active || mini_game.start || mini_game.finish)
    {
        mini_game.start = false;
        mini_game.active = false;
        mini_game.finish = false;
        game_state.set(GameState::InfoScreen).unwrap();
        input.reset(KeyCode::Return);
        return;
    }
    if mini_game.start {
        for (mut text, mut ready) in text_query.iter_mut() {
            if ready.ready_time < 1. {
                if ready.ready_time == 0. {
                    audio.play(asset_library.audio("ready"));
                }
                ready.ready_time += timer.delta_seconds();
                text.sections[0].value = "Ready?".into();
            } else if ready.start_time < 1. {
                if ready.start_time == 0. {
                    audio.play(asset_library.audio("start"));
                }
                ready.start_time += timer.delta_seconds();
                text.sections[0].value = "Start!".into();
            } else {
                text.sections[0].value = "".into();
                mini_game.start = false;
                mini_game.active = true;
            }
        }
    }
    if mini_game.finish {
        for (mut text, mut ready) in text_query.iter_mut() {
            if ready.finish_time < 3. {
                if ready.finish_time == 0. {
                    audio.play(asset_library.audio("finish"));
                }
                ready.finish_time += timer.delta_seconds();
                text.sections[0].value = "Finish!".into();
            } else {
                text.sections[0].value = "".into();
                mini_game.finish = false;
                mini_game.active = false;
                if mini_game.practice {
                    game_state.set(GameState::InfoScreen).unwrap();
                } else {
                    game.duel = false;
                    game_state.set(GameState::Board).unwrap();
                    game.turn += 1;
                }
            }
        }
    }
}

pub fn practice_text(
    mut query: Query<&mut Visibility, With<PracticeText>>,
    mini_game: Res<MiniGame>,
) {
    for mut visibility in query.iter_mut() {
        visibility.is_visible = mini_game.practice;
    }
}

pub fn coin_text(
    mini_game: Res<MiniGame>,
    mut query: Query<(&mut Text, &MiniGameCoinsDisplay)>,
    game: Res<Game>,
) {
    for (mut text, display) in query.iter_mut() {
        if mini_game.active || mini_game.finish {
            if display.mine {
                text.sections[0].value = format!("Coins: {}", mini_game.display_my_coins);
                text.sections[0].style.color = game.my_color;
            } else {
                text.sections[0].value = format!("Coins: {}", mini_game.display_your_coins);
                text.sections[0].style.color = game.your_color;
            }
        } else {
            text.sections[0].value = "".into();
        }
    }
}

pub mod boats;
pub mod duel;
pub mod rps;

pub mod prelude {
    pub use super::{MiniGame, MiniGameEntity, MiniGameFinish, MiniGameState};
}
