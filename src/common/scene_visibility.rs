use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct SceneVisibility(pub GameState);

pub struct SceneVisibilityPlugin;

impl Plugin for SceneVisibilityPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(scene_visibility_update);
    }
}

pub fn scene_visibility_update(
    mut query: Query<(&SceneVisibility, &mut Visibility)>,
    game_state: Res<State<GameState>>,
) {
    for (scene_visibility, mut visibility) in query.iter_mut() {
        visibility.is_visible = scene_visibility.0 == *game_state.current();
    }
}
