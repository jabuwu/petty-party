use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct LoadingEntity;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Loading).with_system(init))
            .add_system_set(SystemSet::on_exit(GameState::Loading).with_system(cleanup));
    }
}

pub fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                    "loading",
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
            });
        })
        .insert(LoadingEntity);
}

pub fn cleanup(mut commands: Commands, query: Query<Entity, With<LoadingEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
