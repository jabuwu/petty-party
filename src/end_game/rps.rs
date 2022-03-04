use super::prelude::*;
use crate::mini_game::rps::{RpsCompare, RpsSelect};
use crate::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::Audio;

pub struct EgRps {
    lost_health: bool,
    my_select: RpsSelect,
    your_select: Option<RpsSelect>,
}

#[derive(Component)]
pub struct EgRpsHand;

#[derive(Component)]
pub struct EgRpsMove;

pub struct EgRpsPlugin;

impl Plugin for EgRpsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(EgRps {
            lost_health: false,
            my_select: RpsSelect::Rock,
            your_select: None,
        })
        .add_system_set(SystemSet::on_enter(EndGameState::Rps).with_system(enter))
        .add_system_set(SystemSet::on_exit(EndGameState::Rps).with_system(exit))
        .add_system_set(SystemSet::on_update(EndGameState::Rps).with_system(update));
    }
}

pub fn enter(
    mut eg_rps: ResMut<EgRps>,
    game: Res<Game>,
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    audio: Res<Audio>,
) {
    audio.play(asset_library.audio("shoot"));
    eg_rps.lost_health = false;
    eg_rps.my_select = RpsSelect::new_rand();
    eg_rps.your_select = None;
    let mut color = game.your_color;
    color.set_a(0.3);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: asset_library.texture_atlas("rps"),
            transform: Transform::from_xyz(-60., 0., 0.3),
            sprite: TextureAtlasSprite {
                color,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(EndGameEntity)
        .insert(EgRpsHand)
        .insert(EgRpsMove);
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: asset_library.texture_atlas("rps"),
            transform: Transform::from_xyz(60., 0., 0.3),
            sprite: TextureAtlasSprite {
                color: game.my_color,
                flip_x: true,
                index: eg_rps.my_select.to_index(),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(EndGameEntity)
        .insert(EgRpsMove);
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "Shoot!",
                TextStyle {
                    font: asset_library.font("game"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    vertical: VerticalAlign::Center,
                    horizontal: HorizontalAlign::Center,
                },
            ),
            transform: Transform::from_xyz(0.0, 55.0, 0.5),
            ..Default::default()
        })
        .insert(EndGameEntity);
}

pub fn exit(mut commands: Commands, query: Query<Entity, With<EndGameEntity>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn update(
    mut end_game: ResMut<EndGame>,
    mut eg_rps: ResMut<EgRps>,
    mut query: Query<&mut TextureAtlasSprite, With<EgRpsHand>>,
    mut move_query: Query<&mut Transform, With<EgRpsMove>>,
    input: ResMut<Input<KeyCode>>,
    audio: Res<Audio>,
    asset_library: Res<AssetLibrary>,
) {
    for mut sprite in query.iter_mut() {
        if let Some(select) = eg_rps.your_select {
            sprite.color.set_a(1.);
            sprite.index = select.to_index();
        } else {
            sprite.color.set_a(0.3);
        }
    }
    if eg_rps.your_select.is_none() {
        if input.just_pressed(KeyCode::R) {
            audio.play(asset_library.audio("shoot"));
            eg_rps.your_select = Some(RpsSelect::Rock);
        } else if input.just_pressed(KeyCode::P) {
            audio.play(asset_library.audio("shoot"));
            eg_rps.your_select = Some(RpsSelect::Paper);
        } else if input.just_pressed(KeyCode::S) {
            audio.play(asset_library.audio("shoot"));
            eg_rps.your_select = Some(RpsSelect::Scissors);
        }
    }
    for mut transform in move_query.iter_mut() {
        if end_game.state_time < 0.2 {
            transform.translation.y = (end_game.state_time * 15.).cos() * 10.;
        }
    }
    if end_game.state_time > 2.5 {
        if !eg_rps.lost_health {
            let your_select = if let Some(select) = eg_rps.your_select {
                select
            } else {
                RpsSelect::Rock
            };
            match your_select.compare(eg_rps.my_select) {
                RpsCompare::Lose => {
                    if end_game.your_health > 0 {
                        end_game.your_health -= 1;
                    }
                }
                RpsCompare::Draw => {
                    end_game.turn -= 1;
                }
                RpsCompare::Win => {}
            }
            eg_rps.lost_health = true;
        }
        end_game.switch = true;
    }
}
