use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct SetupEntity;

#[derive(Component)]
pub struct SetupText;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Setup).with_system(enter))
            .add_system_set(SystemSet::on_exit(GameState::Setup).with_system(exit))
            .add_system_set(SystemSet::on_update(GameState::Setup).with_system(update));
    }
}

pub fn enter(mut commands: Commands, asset_library: Res<AssetLibrary>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
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
            parent
                .spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "",
                        TextStyle {
                            font: asset_library.font("game"),
                            font_size: 32.0,
                            color: Color::WHITE,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            vertical: VerticalAlign::Center,
                        },
                    ),
                    ..Default::default()
                })
                .insert(SetupText);
        })
        .insert(SetupEntity);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Vec2::new(32., 32.).into(),
                color: Color::CYAN,
                ..Default::default()
            },
            transform: Transform::from_xyz(-70., 15., 0.),
            ..Default::default()
        })
        .insert(SetupEntity);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Vec2::new(32., 32.).into(),
                color: Color::YELLOW,
                ..Default::default()
            },
            transform: Transform::from_xyz(0., 15., 0.),
            ..Default::default()
        })
        .insert(SetupEntity);
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                custom_size: Vec2::new(32., 32.).into(),
                color: Color::ORANGE,
                ..Default::default()
            },
            transform: Transform::from_xyz(70., 15., 0.),
            ..Default::default()
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
    mut text_query: Query<&mut Text, With<SetupText>>,
    mut difficulty: ResMut<Difficulty>,
) {
    if input.just_pressed(KeyCode::Y) {
        game.your_color = Color::YELLOW;
        game_state.set(GameState::Intro).unwrap();
        *difficulty = Difficulty::Normal;
    } else if input.just_pressed(KeyCode::B) {
        game.your_color = Color::CYAN;
        game_state.set(GameState::Intro).unwrap();
        *difficulty = Difficulty::Normal;
    } else if input.just_pressed(KeyCode::O) {
        game.your_color = Color::ORANGE;
        *difficulty = Difficulty::Normal;
        game_state.set(GameState::Intro).unwrap();
    } else if input.just_pressed(KeyCode::Key1) {
        game.your_color = Color::ORANGE_RED;
        game_state.set(GameState::Intro).unwrap();
        *difficulty = Difficulty::Hard;
    }
    for mut text in text_query.iter_mut() {
        text.sections[0].value =
            "Select Your Color:\n\n\n\n\nB - Blue\nY - Yellow\nO - Orange".into();
    }
}
