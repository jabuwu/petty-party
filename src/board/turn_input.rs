use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct LocalEntity;

pub struct TurnInputPlugin;

impl Plugin for TurnInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(BoardState::TurnInput).with_system(init))
            .add_system_set(SystemSet::on_exit(BoardState::TurnInput).with_system(cleanup))
            .add_system_set(SystemSet::on_update(BoardState::TurnInput).with_system(update));
    }
}

pub fn init(mut commands: Commands, asset_library: Res<AssetLibrary>, board: Res<Board>) {
    if !board.my_turn {
        let item_text = match board.your_item {
            Item::None => "",
            Item::Rapier => "\nR - Use Rapier",
            Item::CrystalBall => "\nC - Use Crystal Ball",
            Item::TrumpCard => "\nT - Use Trump Card",
            Item::TestItem => "\nT - Use Test Item",
        };
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        bottom: Val::Px(0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    text: Text::with_section(
                        format!("SPACE - Roll Dice\nF - Free Cam{}", item_text),
                        TextStyle {
                            font: asset_library.font("game"),
                            font_size: 24.0,
                            color: Color::BLACK,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            vertical: VerticalAlign::Center,
                        },
                    ),
                    ..Default::default()
                });
            })
            .insert(LocalEntity);
    }
}

pub fn cleanup(mut commands: Commands, local_query: Query<Entity, With<LocalEntity>>) {
    for entity in local_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update(
    board: Res<Board>,
    mut input: ResMut<Input<KeyCode>>,
    mut board_state: ResMut<State<BoardState>>,
    dialogue: Res<Dialogue>,
) {
    if dialogue.busy() {
        return;
    }
    if board.my_turn {
        board_state.set(BoardState::Moving).unwrap();
    } else {
        if input.just_pressed(KeyCode::Space) {
            board_state.set(BoardState::Moving).unwrap();
            input.reset(KeyCode::Space);
        } else if input.just_pressed(KeyCode::F) {
            board_state.set(BoardState::FreeCam).unwrap();
            input.reset(KeyCode::F);
        } else if input.just_pressed(KeyCode::R) {
            if matches!(board.your_item, Item::Rapier) {
                board_state.set(BoardState::UseItem).unwrap();
            }
        } else if input.just_pressed(KeyCode::C) {
            if matches!(board.your_item, Item::CrystalBall) {
                board_state.set(BoardState::UseItem).unwrap();
            }
        }
    }
}
