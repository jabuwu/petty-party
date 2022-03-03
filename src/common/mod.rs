use asset_library::AssetLibraryPlugin;
use bevy::prelude::*;
use camera_controller::CameraControllerPlugin;
use collision::CollisionPlugin;
use dialogue::DialoguePlugin;
use dice_roll::DiceRollPlugin;
use scene_visibility::SceneVisibilityPlugin;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AssetLibraryPlugin)
            .add_plugin(CollisionPlugin)
            .add_plugin(SceneVisibilityPlugin)
            .add_plugin(CameraControllerPlugin)
            .add_plugin(DialoguePlugin)
            .add_plugin(DiceRollPlugin);
    }
}

pub mod animation;
pub mod asset_library;
pub mod camera_controller;
pub mod collision;
pub mod collision_shape;
pub mod dialogue;
pub mod dice_roll;
pub mod scene_visibility;

pub mod prelude {
    pub use super::{
        animation::Animation,
        asset_library::{AssetLibrary, AssetLibraryReady},
        camera_controller::CameraController,
        collision::{Collision, CollisionFilter, CollisionQuery},
        collision_shape::CollisionShape,
        dialogue::{Dialogue, DialogueEntry},
        dice_roll::{DiceRollEnd, DiceRollHide, DiceRollStart, DiceRollValue},
        scene_visibility::SceneVisibility,
    };
}
