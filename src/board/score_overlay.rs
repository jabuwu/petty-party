use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct ScoreOverlay;

#[derive(Component)]
pub struct ScoreOverlayCoins {
    mine: bool,
}

#[derive(Component)]
pub struct ScoreOverlayBg {
    mine: bool,
}

#[derive(Component)]
pub struct ScoreOverlayItem {
    mine: bool,
}

pub struct ScoreOverlayPlugin;

impl Plugin for ScoreOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(init)
            .add_system(update)
            .add_system(update_coins)
            .add_system(update_items);
    }
}

pub fn init(
    game: Res<Game>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut asset_library_ready: EventReader<AssetLibraryReady>,
) {
    for _ in asset_library_ready.iter() {
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::SpaceBetween,
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
                            position_type: PositionType::Absolute,
                            position: Rect {
                                top: Val::Px(0.),
                                left: Val::Px(0.),
                                ..Default::default()
                            },
                            size: Size::new(Val::Px(640.0), Val::Px(480.0)),
                            ..Default::default()
                        },
                        image: asset_server.load("sprites/score_overlay_1.png").into(),
                        color: UiColor(game.your_color),
                        ..Default::default()
                    })
                    .insert(ScoreOverlayBg { mine: false });
                parent
                    .spawn_bundle(ImageBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            position: Rect {
                                top: Val::Px(0.),
                                left: Val::Px(0.),
                                ..Default::default()
                            },
                            size: Size::new(Val::Px(640.0), Val::Px(480.0)),
                            ..Default::default()
                        },
                        color: UiColor(game.my_color),
                        image: asset_server.load("sprites/score_overlay_2.png").into(),
                        ..Default::default()
                    })
                    .insert(ScoreOverlayBg { mine: true });
                parent
                    .spawn_bundle(ImageBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            position: Rect {
                                top: Val::Px(0.),
                                left: Val::Px(0.),
                                ..Default::default()
                            },
                            size: Size::new(Val::Px(640.0), Val::Px(480.0)),
                            ..Default::default()
                        },
                        visibility: Visibility { is_visible: false },
                        image: asset_server.load("sprites/score_overlay_3.png").into(),
                        ..Default::default()
                    })
                    .insert(ScoreOverlay)
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(TextBundle {
                                style: Style {
                                    align_self: AlignSelf::Center,
                                    position_type: PositionType::Absolute,
                                    position: Rect {
                                        left: Val::Px(150.0),
                                        top: Val::Px(35.0),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                visibility: Visibility { is_visible: false },
                                text: Text::with_section(
                                    "",
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
                            .insert(ScoreOverlayCoins { mine: false });
                        parent
                            .spawn_bundle(TextBundle {
                                style: Style {
                                    align_self: AlignSelf::Center,
                                    position_type: PositionType::Absolute,
                                    position: Rect {
                                        right: Val::Px(150.0),
                                        top: Val::Px(35.0),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                visibility: Visibility { is_visible: false },
                                text: Text::with_section(
                                    "",
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
                            .insert(ScoreOverlayCoins { mine: true });
                    });
                parent
                    .spawn_bundle(ImageBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            position: Rect {
                                top: Val::Px(80.),
                                left: Val::Px(210.),
                                ..Default::default()
                            },
                            size: Size::new(Val::Px(32.0), Val::Px(32.0)),
                            ..Default::default()
                        },
                        visibility: Visibility { is_visible: false },
                        image: asset_server.load("sprites/item_rapier.png").into(),
                        ..Default::default()
                    })
                    .insert(ScoreOverlayItem { mine: false });
                parent
                    .spawn_bundle(ImageBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            position: Rect {
                                top: Val::Px(80.),
                                right: Val::Px(210.),
                                ..Default::default()
                            },
                            size: Size::new(Val::Px(32.0), Val::Px(32.0)),
                            ..Default::default()
                        },
                        visibility: Visibility { is_visible: false },
                        image: asset_server.load("sprites/item_mystery.png").into(),
                        ..Default::default()
                    })
                    .insert(ScoreOverlayItem { mine: true });
            });
    }
}

pub fn update(board: Res<Board>, mut query: Query<&mut Visibility, With<ScoreOverlay>>) {
    for mut visibility in query.iter_mut() {
        visibility.is_visible = board.score_overlay;
    }
}

pub fn update_coins(
    game: Res<Game>,
    board: Res<Board>,
    mut query: Query<(&mut Visibility, &mut Text, &ScoreOverlayCoins)>,
    mut query_bg: Query<(&mut UiColor, &ScoreOverlayBg)>,
) {
    for (mut visibility, mut text, coins) in query.iter_mut() {
        visibility.is_visible = board.score_overlay;
        if coins.mine {
            text.sections[0].value = format!("{}", game.my_coins);
        }
        if !coins.mine {
            text.sections[0].value = format!("{}", game.your_coins);
        }
    }
    for (mut color, bg) in query_bg.iter_mut() {
        if board.score_overlay {
            if bg.mine {
                color.0 = game.my_color;
            } else {
                color.0 = game.your_color;
            }
        } else {
            color.0 = Color::rgba(0., 0., 0., 0.);
        }
    }
}

pub fn update_items(
    board: Res<Board>,
    mut query: Query<(&mut Visibility, &mut UiImage, &ScoreOverlayItem)>,
    asset_library: Res<AssetLibrary>,
) {
    for (mut visibility, mut image, item) in query.iter_mut() {
        if board.score_overlay {
            if item.mine {
                image.0 = asset_library.image("item_mystery");
                visibility.is_visible = true;
            } else {
                if matches!(board.your_item, Item::Rapier) {
                    image.0 = asset_library.image("item_rapier");
                    visibility.is_visible = true;
                } else {
                    visibility.is_visible = false;
                }
            }
        } else {
            visibility.is_visible = false;
        }
    }
}
