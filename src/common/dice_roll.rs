use crate::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel};

pub struct DiceState {
    visible: bool,
    time: f32,
    value: u32,
}

pub struct DiceRollStart {
    pub value: u32,
}

pub struct DiceRollEnd;

pub struct DiceRollValue {
    pub value: u32,
}

pub struct DiceRollHide;

#[derive(Component)]
pub struct Dice;

pub struct DiceRollPlugin;

impl Plugin for DiceRollPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DiceState {
            visible: false,
            time: 0.,
            value: 1,
        })
        .add_event::<DiceRollStart>()
        .add_event::<DiceRollEnd>()
        .add_event::<DiceRollValue>()
        .add_event::<DiceRollHide>()
        .add_system(init)
        .add_system(update)
        .add_system_to_stage(CoreStage::PostUpdate, follow_camera.after("update_camera"));
    }
}

pub fn init(
    mut commands: Commands,
    asset_library: Res<AssetLibrary>,
    mut asset_library_ready: EventReader<AssetLibraryReady>,
) {
    for _ in asset_library_ready.iter() {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: asset_library.texture_atlas("dice_roll"),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                visibility: Visibility { is_visible: false },
                ..Default::default()
            })
            .insert(Dice);
    }
}

pub struct AudioState {
    roll: AudioChannel,
    rolling: bool,
}

impl Default for AudioState {
    fn default() -> Self {
        Self {
            roll: AudioChannel::new("roll".into()),
            rolling: false,
        }
    }
}

pub fn update(
    mut game: ResMut<Game>,
    mut dice: ResMut<DiceState>,
    mut dice_start: EventReader<DiceRollStart>,
    mut dice_end: EventWriter<DiceRollEnd>,
    mut dice_hide: EventReader<DiceRollHide>,
    mut dice_value: EventReader<DiceRollValue>,
    mut dice_query: Query<(&mut TextureAtlasSprite, &mut Visibility), With<Dice>>,
    time: Res<Time>,
    audio: Res<Audio>,
    asset_library: Res<AssetLibrary>,
    mut audio_state: Local<AudioState>,
) {
    for event in dice_start.iter() {
        game.dice_roll = true;
        dice.visible = true;
        dice.time = 0.;
        dice.value = event.value;
        audio.play_looped_in_channel(asset_library.audio("diceroll"), &audio_state.roll);
        audio_state.rolling = true;
    }
    for _ in dice_hide.iter() {
        dice.visible = false;
        audio.stop_channel(&audio_state.roll);
    }
    for event in dice_value.iter() {
        dice.value = event.value;
    }
    if dice.visible {
        if dice.time < 1.5 {
            dice.time += time.delta_seconds();
            if dice.time >= 1.5 {
                dice_end.send(DiceRollEnd);
                game.dice_roll = false;
            }
        }
    }
    for (mut sprite, mut visibility) in dice_query.iter_mut() {
        if dice.visible {
            visibility.is_visible = true;
            if dice.time >= 1. {
                if audio_state.rolling {
                    audio.play(asset_library.audio("diceding"));
                    audio.stop_channel(&audio_state.roll);
                    audio_state.rolling = false;
                }
                sprite.index = (4 + dice.value) as usize;
            } else {
                sprite.index = ((dice.time * 15.) as usize) % 4;
            }
        } else {
            visibility.is_visible = false;
        }
    }
}

pub fn follow_camera(
    mut query: QuerySet<(
        QueryState<&Transform, With<GameCamera>>,
        QueryState<&mut Transform, With<Dice>>,
    )>,
) {
    let mut camera_position = Vec2::new(0., 0.);
    for transform in query.q0().iter() {
        camera_position = transform.translation.truncate();
    }
    camera_position.y += 30.;
    for mut dice_transform in query.q1().iter_mut() {
        dice_transform.translation = camera_position.extend(0.9);
    }
}

pub fn reset(mut dice: ResMut<DiceState>, mut reset_event: EventReader<GameReset>) {
    for _ in reset_event.iter() {
        dice.visible = false;
        dice.time = 0.;
    }
}
