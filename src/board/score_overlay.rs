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
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
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
                        image: asset_library.image("score_overlay_1").into(),
                        color: UiColor(Color::NONE),
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
                        color: UiColor(Color::NONE),
                        image: asset_library.image("score_overlay_2").into(),
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
                        image: asset_library.image("score_overlay_3").into(),
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
                                        top: Val::Px(39.0),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                visibility: Visibility { is_visible: false },
                                text: Text::with_section(
                                    "",
                                    TextStyle {
                                        font: asset_library.font("game"),
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
                                        top: Val::Px(39.0),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                visibility: Visibility { is_visible: false },
                                text: Text::with_section(
                                    "",
                                    TextStyle {
                                        font: asset_library.font("game"),
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
                        image: asset_library.image("item_rapier").into(),
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
                        image: asset_library.image("item_mystery").into(),
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
    mut board: ResMut<Board>,
    mut query: Query<(
        &mut Visibility,
        &mut UiImage,
        &mut Style,
        &mut UiColor,
        &ScoreOverlayItem,
    )>,
    asset_library: Res<AssetLibrary>,
    input: Res<Input<KeyCode>>,
) {
    if input.pressed(KeyCode::Key5) {
        board.my_item_use_interpolate -= 0.025;
    }
    if input.pressed(KeyCode::Key6) {
        board.my_item_use_interpolate += 0.025;
    }
    if input.pressed(KeyCode::Key7) {
        board.your_item_use_interpolate -= 0.025;
    }
    if input.pressed(KeyCode::Key8) {
        board.your_item_use_interpolate += 0.025;
    }
    board.my_item_use_interpolate = board.my_item_use_interpolate.clamp(0., 1.);
    board.your_item_use_interpolate = board.your_item_use_interpolate.clamp(0., 1.);
    for (mut visibility, mut image, mut style, mut color, item) in query.iter_mut() {
        if board.score_overlay {
            if item.mine {
                let interp = board.my_item_use_interpolate;
                let mut move_interp = (interp * 1.25).min(1.);
                move_interp *= move_interp;
                let color_interp = board.my_item_use_interpolate.powf(10.);
                style.position.top = Val::Px(80. + board.my_item_use_interpolate * 120.);
                style.position.right = Val::Px(210. + move_interp * 55.);
                style.size = Size::new(
                    Val::Px(32.0 + 64. * move_interp),
                    Val::Px(32.0 + 64. * move_interp),
                );
                color.0 = Color::rgba(1., 1., 1. - color_interp, 1. - color_interp);
                visibility.is_visible = true;
                image.0 = asset_library.image("item_mystery");
            } else {
                let interp = board.your_item_use_interpolate;
                let mut move_interp = (interp * 1.25).min(1.);
                move_interp *= move_interp;
                let color_interp = board.your_item_use_interpolate.powf(10.);
                style.position.top = Val::Px(80. + board.your_item_use_interpolate * 120.);
                style.position.left = Val::Px(210. + move_interp * 55.);
                style.size = Size::new(
                    Val::Px(32.0 + 64. * move_interp),
                    Val::Px(32.0 + 64. * move_interp),
                );
                color.0 = Color::rgba(1., 1., 1. - color_interp, 1. - color_interp);
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
