use crate::prelude::*;
use bevy::prelude::*;

pub struct Setup {}

#[derive(Component)]
pub struct SetupEntity;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Setup {})
            .add_system_set(SystemSet::on_enter(GameState::Setup).with_system(enter))
            .add_system_set(SystemSet::on_exit(GameState::Setup).with_system(exit))
            .add_system_set(SystemSet::on_update(GameState::Setup).with_system(update));
    }
}

pub fn enter(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(80.0)),
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
                    "Select Your Color:\n\nB - Blue\nY - Yellow\nO - Orange",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 32.0,
                        color: Color::WHITE,
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        vertical: VerticalAlign::Center,
                    },
                ),
                ..Default::default()
            });
        })
        .insert(SetupEntity);
}

pub fn exit(mut commands: Commands, query: Query<Entity, With<SetupEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update(
    mut game: ResMut<Game>,
    input: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    mut asset_library: ResMut<AssetLibrary>,
) {
    if input.just_pressed(KeyCode::Y) {
        asset_library.load_audio(&asset_server);
        game.your_color = Color::YELLOW;
        game_state.set(GameState::Intro).unwrap();
    } else if input.just_pressed(KeyCode::B) {
        asset_library.load_audio(&asset_server);
        game.your_color = Color::CYAN;
        game_state.set(GameState::Intro).unwrap();
    } else if input.just_pressed(KeyCode::O) {
        asset_library.load_audio(&asset_server);
        game.your_color = Color::ORANGE;
        game_state.set(GameState::Intro).unwrap();
    }
}
