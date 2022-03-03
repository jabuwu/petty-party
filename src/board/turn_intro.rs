use crate::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::Audio;

#[derive(Component)]
pub struct LocalEntity;

#[derive(Default)]
pub struct TurnIntro {
    animation: Option<Animation>,
}

pub struct TurnIntroPlugin;

impl Plugin for TurnIntroPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TurnIntro { animation: None })
            .add_system_set(SystemSet::on_enter(BoardState::TurnIntro).with_system(init))
            .add_system_set(SystemSet::on_exit(BoardState::TurnIntro).with_system(cleanup))
            .add_system_set(SystemSet::on_update(BoardState::TurnIntro).with_system(update));
    }
}

pub fn init(
    mut turn_intro: ResMut<TurnIntro>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut board: ResMut<Board>,
    mut camera_controller: ResMut<CameraController>,
    audio: Res<Audio>,
    asset_library: Res<AssetLibrary>,
) {
    audio.play(asset_library.audio("turnstart"));
    if board.my_turn {
        board.active_pawn = board.my_pawn;
    } else {
        board.active_pawn = board.your_pawn;
    }
    turn_intro.animation = Some(Animation::time(1.0));
    if let Some(active_pawn) = board.active_pawn {
        camera_controller.follow_entity = Some(active_pawn);
    }

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
            parent.spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::Center,
                    position_type: PositionType::Relative,
                    position: Rect {
                        top: Val::Px(-50.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::with_section(
                    if board.my_turn {
                        "My Turn"
                    } else {
                        "Your Turn"
                    },
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 42.0,
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
    mut turn_intro: ResMut<TurnIntro>,
    mut commands: Commands,
    local_query: Query<Entity, With<LocalEntity>>,
) {
    turn_intro.animation = None;
    for entity in local_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update(
    mut turn_intro: ResMut<TurnIntro>,
    time: Res<Time>,
    mut board_state: ResMut<State<BoardState>>,
) {
    if let Some(animation) = &mut turn_intro.animation {
        animation.update(time.delta_seconds());
        if animation.finished() {
            board_state.set(BoardState::TurnInput).unwrap();
        }
    }
}
