use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct LocalEntity;

pub struct InfoScreen {
    active: bool,
    mini_game: MiniGameState,
}

#[derive(Component)]
pub struct InfoScreenText;

pub struct InfoScreenPlugin;

impl Plugin for InfoScreenPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(InfoScreen {
            active: false,
            mini_game: MiniGameState::Inactive,
        })
        .add_system_set(SystemSet::on_enter(GameState::InfoScreen).with_system(enter))
        .add_system_set(SystemSet::on_exit(GameState::InfoScreen).with_system(exit))
        .add_system(update);
    }
}

pub fn enter(
    game: Res<Game>,
    mut info_screen: ResMut<InfoScreen>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    info_screen.active = true;
    if game.duel {
        info_screen.mini_game = MiniGameState::Duel;
    } else {
        info_screen.mini_game = match game.turn % 2 {
            0 => MiniGameState::Rps,
            _ => MiniGameState::Boats,
        };
    }
    let mut mini_game_name = String::new();
    let mut mini_game_description = String::new();
    match info_screen.mini_game {
        MiniGameState::Boats => {
            mini_game_name = "Cannon Ball Dodge".into();
            mini_game_description =
                "Dodge the falling cannon balls!\nStart with 6 coins!\nIf you get hit you lose 2 coins!\nGame goes until someone runs out of coins!\n\nControls:\nWASD - Move".into();
        }
        MiniGameState::Rps => {
            mini_game_name = "Rock Paper Scissors".into();
            mini_game_description =
                "Rock, Paper, Scissors, SHOOT!\nReact to your opponent!\nMake your selection quickly AFTER them!\nPlay 3 rounds, round winner gets 2 coins!\n\nControls:\nR - Rock\nP - Paper\nS - Scissors".into();
        }
        MiniGameState::Duel => {
            mini_game_name = "Duel".into();
            mini_game_description =
                "Attack your opponent! Defend against their attack!\nDefend right before an attack to stun your opponent!\nCancel your attack by releasing the A button!\nBait your opponent into defending prematurely!\nSteal coins if you successfully attack them.\n\nControls:\nA - Hold to attack\nD - Hold to defend".into();
        }
        MiniGameState::Inactive => {}
    }
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(ImageBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::SpaceAround,
                        position_type: PositionType::Absolute,
                        flex_direction: FlexDirection::Column,
                        ..Default::default()
                    },
                    image: asset_library.image("info_bg").into(),
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
                                "Press SPACE to start\nPress ENTER to practice",
                                TextStyle {
                                    font: asset_library.font("game"),
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
                        .insert(InfoScreenText);
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                align_self: AlignSelf::Center,
                                ..Default::default()
                            },
                            text: Text::with_section(
                                mini_game_description,
                                TextStyle {
                                    font: asset_library.font("game"),
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
                        .insert(InfoScreenText);
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                align_self: AlignSelf::Center,
                                margin: Rect {
                                    top: Val::Px(10.),
                                    bottom: Val::Px(10.),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            text: Text::with_section(
                                mini_game_name,
                                TextStyle {
                                    font: asset_library.font("game"),
                                    font_size: 42.0,
                                    color: Color::WHITE,
                                },
                                TextAlignment {
                                    horizontal: HorizontalAlign::Left,
                                    vertical: VerticalAlign::Top,
                                },
                            ),
                            ..Default::default()
                        })
                        .insert(InfoScreenText);
                });
        })
        .insert(LocalEntity);
}

pub fn exit(
    mut info_screen: ResMut<InfoScreen>,
    mut commands: Commands,
    destroy_query: Query<Entity, With<LocalEntity>>,
) {
    info_screen.active = false;
    for entity in destroy_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update(
    mut game: ResMut<Game>,
    info_screen: Res<InfoScreen>,
    mut mini_game: ResMut<MiniGame>,
    mut input: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
    mut mini_game_state: ResMut<State<MiniGameState>>,
    mut dialogue: ResMut<Dialogue>,
    mut text_query: Query<&mut Visibility, With<InfoScreenText>>,
) {
    if dialogue.busy() {
        for mut visibility in text_query.iter_mut() {
            visibility.is_visible = false;
        }
        return;
    }
    for mut visibility in text_query.iter_mut() {
        visibility.is_visible = true;
    }
    if info_screen.active && !dialogue.busy() {
        if input.just_pressed(KeyCode::Space) {
            if game.practice_first_message {
                dialogue.add(DialogueEntry {
                    text: "You should practice first!".into(),
                    ..Default::default()
                });
                dialogue.add(DialogueEntry {
                    text: "Press ENTER instead of SPACE to practice the mini game.".into(),
                    ..Default::default()
                });
                game.practice_first_message = false;
            } else {
                mini_game.practice = false;
                mini_game_state.set(info_screen.mini_game).unwrap();
                game_state.set(GameState::MiniGame).unwrap();
            }
            input.reset(KeyCode::Space);
        }
        if input.just_pressed(KeyCode::Return) {
            mini_game.practice = true;
            mini_game_state.set(info_screen.mini_game).unwrap();
            game_state.set(GameState::MiniGame).unwrap();
            input.reset(KeyCode::Return);
        }
    }
}
