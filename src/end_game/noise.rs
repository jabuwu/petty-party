use super::prelude::*;
use crate::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel};
use rand::prelude::*;

pub struct AudioState {
    channel: AudioChannel,
}

impl Default for AudioState {
    fn default() -> Self {
        Self {
            channel: AudioChannel::new("noise".into()),
        }
    }
}

#[derive(Component)]
pub struct Noise;

pub struct NoisePlugin;

impl Plugin for NoisePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioState>()
            .add_system_set(SystemSet::on_enter(EndGameState::Noise).with_system(enter))
            .add_system_set(SystemSet::on_update(EndGameState::Noise).with_system(update))
            .add_system_set(SystemSet::on_exit(EndGameState::Noise).with_system(exit));
    }
}

pub fn enter(
    mut commands: Commands,
    audio: Res<Audio>,
    asset_library: Res<AssetLibrary>,
    audio_state: Res<AudioState>,
) {
    audio.play_looped_in_channel(asset_library.audio("static"), &audio_state.channel);
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_library.image("static"),
            ..Default::default()
        })
        .insert(Noise);
}

pub fn update(mut noise_query: Query<&mut Transform, With<Noise>>) {
    let mut rng = rand::thread_rng();
    for mut transform in noise_query.iter_mut() {
        transform.translation.x = rng.gen_range(-160.0..160.0f32);
        transform.translation.y = rng.gen_range(-120.0..120.0f32);
    }
}

pub fn exit(
    mut commands: Commands,
    noise_query: Query<Entity, With<Noise>>,
    audio: Res<Audio>,
    audio_state: Res<AudioState>,
) {
    audio.stop_channel(&audio_state.channel);
    for entity in noise_query.iter() {
        commands.entity(entity).despawn();
    }
}
