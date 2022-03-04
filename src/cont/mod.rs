use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct ContinueEntity;

#[derive(Component)]
pub struct ContinueText;

pub struct ContinuePlugin;

impl Plugin for ContinuePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Continue).with_system(enter))
            .add_system_set(SystemSet::on_exit(GameState::Continue).with_system(exit))
            .add_system_set(SystemSet::on_update(GameState::Continue).with_system(update));
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
                .insert(ContinueText);
        })
        .insert(ContinueEntity);
}

pub fn exit(mut commands: Commands, query: Query<Entity, With<ContinueEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update(
    input: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
    mut text_query: Query<&mut Text, With<ContinueText>>,
    mut reset: EventWriter<GameResetSend>,
) {
    if input.just_pressed(KeyCode::Y) {
        game_state.set(GameState::EndGame).unwrap();
    } else if input.just_pressed(KeyCode::N) {
        reset.send(GameResetSend);
    }
    for mut text in text_query.iter_mut() {
        text.sections[0].value = "Game Over\n\nContinue?\nY / N".into();
    }
}
