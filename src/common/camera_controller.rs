use crate::prelude::*;
use bevy::prelude::*;

pub struct CameraController {
    pub center: bool,
    pub zoom_out: bool,
    pub follow_entity: Option<Entity>,
}

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraController {
            center: false,
            zoom_out: false,
            follow_entity: None,
        })
        .add_system_to_stage(CoreStage::PostUpdate, update_camera.label("update_camera"));
    }
}

pub fn update_camera(
    mut camera_controller: ResMut<CameraController>,
    mut queries: QuerySet<(
        QueryState<&Transform>,
        QueryState<&mut Transform, With<GameCamera>>,
    )>,
) {
    if camera_controller.center {
        for mut camera_transform in queries.q1().iter_mut() {
            camera_transform.translation.x = 0.;
            camera_transform.translation.y = 0.;
        }
        camera_controller.center = false;
    } else {
        let target_position = if let Some(follow_entity) = camera_controller.follow_entity {
            if let Ok(transform) = queries.q0().get(follow_entity) {
                Some(transform.translation.truncate())
            } else {
                None
            }
        } else {
            None
        };
        if let Some(target_position) = target_position {
            for mut camera_transform in queries.q1().iter_mut() {
                camera_transform.translation.x = target_position.x;
                camera_transform.translation.y = target_position.y;
            }
        }
    }
    for mut camera_transform in queries.q1().iter_mut() {
        if camera_controller.zoom_out {
            camera_transform.scale.x = 1.;
            camera_transform.scale.y = 1.;
        } else {
            camera_transform.scale.x = 0.5;
            camera_transform.scale.y = 0.5;
        }
    }
}

pub fn reset(mut reset: EventReader<GameReset>, mut camera_controller: ResMut<CameraController>) {
    for _ in reset.iter() {
        camera_controller.center = true;
        camera_controller.zoom_out = false;
        camera_controller.follow_entity = None;
    }
}
