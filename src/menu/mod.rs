use crate::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::Audio;

pub struct MenuPlugin;

#[derive(Component)]
pub struct MenuEntity;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Menu).with_system(enter))
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(exit))
            .add_system_set(SystemSet::on_update(GameState::Menu).with_system(update));
    }
}

pub fn enter(mut commands: Commands, asset_library: Res<AssetLibrary>) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_library.image("menu_bg"),
            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::new(0.5, 0.5, 1.)),
            ..Default::default()
        })
        .insert(MenuEntity);
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_library.image("menu_logo"),
            transform: Transform::from_xyz(0., 10., 0.1).with_scale(Vec3::new(0.5, 0.5, 1.)),
            ..Default::default()
        })
        .insert(MenuEntity);
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "Press SPACE to Play",
                TextStyle {
                    color: Color::BLACK,
                    font: asset_library.font("game"),
                    font_size: 24.,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            transform: Transform::from_xyz(0., -80., 0.3),
            ..Default::default()
        })
        .insert(MenuEntity);
}

pub fn exit(mut commands: Commands, query: Query<Entity, With<MenuEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn update(
    input: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
    asset_library: Res<AssetLibrary>,
    audio: Res<Audio>,
) {
    if input.just_pressed(KeyCode::Space) {
        game_state.set(GameState::Setup).unwrap();
        audio.play(asset_library.audio("dialogue"));
    }
}
