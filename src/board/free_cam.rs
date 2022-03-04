use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct LocalEntity;

pub struct FreeCamPlugin;

impl Plugin for FreeCamPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(BoardState::FreeCam).with_system(init))
            .add_system_set(SystemSet::on_exit(BoardState::FreeCam).with_system(cleanup))
            .add_system_set(SystemSet::on_update(BoardState::FreeCam).with_system(update))
            .add_system_set(SystemSet::on_update(BoardState::FreeCam).with_system(move_camera));
    }
}

pub fn init(
    mut board: ResMut<Board>,
    mut camera_controller: ResMut<CameraController>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
) {
    board.score_overlay = false;
    camera_controller.follow_entity = None;
    camera_controller.zoom_out = true;
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
                    "WASD - Look Around\nF - Exit Free Cam",
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

pub fn cleanup(
    mut board: ResMut<Board>,
    mut camera_controller: ResMut<CameraController>,
    mut commands: Commands,
    local_query: Query<Entity, With<LocalEntity>>,
) {
    if let Some(active_pawn) = board.active_pawn {
        camera_controller.follow_entity = Some(active_pawn);
    }
    camera_controller.zoom_out = false;
    board.score_overlay = true;
    for entity in local_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update(mut input: ResMut<Input<KeyCode>>, mut board_state: ResMut<State<BoardState>>) {
    if input.just_pressed(KeyCode::F) {
        board_state.set(BoardState::TurnInput).unwrap();
        input.reset(KeyCode::F);
    }
}

pub fn move_camera(
    mut camera_query: Query<&mut Transform, With<GameCamera>>,
    input: Res<Input<KeyCode>>,
) {
    let mut movement = Vec2::ZERO;
    if input.pressed(KeyCode::S) {
        movement.y -= 1.;
    }
    if input.pressed(KeyCode::W) {
        movement.y += 1.;
    }
    if input.pressed(KeyCode::A) {
        movement.x -= 1.;
    }
    if input.pressed(KeyCode::D) {
        movement.x += 1.;
    }
    movement = movement.normalize_or_zero() * 10.;
    for mut transform in camera_query.iter_mut() {
        transform.translation.x += movement.x;
        transform.translation.y += movement.y;
        transform.translation.x = transform.translation.x.clamp(-240., 240.);
        transform.translation.y = transform.translation.y.clamp(-240., 270.);
    }
}
