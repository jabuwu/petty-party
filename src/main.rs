use bevy::prelude::*;
use bevy_kira_audio::AudioPlugin;
use board::BoardPlugin;
use common::asset_library::AssetLibraryReady;
use common::CommonPlugin;
use info_screen::InfoScreenPlugin;
use intro::IntroPlugin;
use loading::LoadingPlugin;
use mini_game::MiniGamePlugin;
use setup::SetupPlugin;

pub enum Difficulty {
    Easy,
    Normal,
    Hard,
    VeryHard,
}

#[derive(Default)]
pub struct Game {
    pub my_color: Color,
    pub your_color: Color,
    pub my_coins: u32,
    pub your_coins: u32,
    pub turn: u32,
    pub reset_timer: f32,
    pub duel: bool,
    pub dice_roll: bool,
}

pub struct GameReset;

pub struct GameResetSend;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum GameState {
    Loading,
    Setup,
    Intro,
    Board,
    InfoScreen,
    MiniGame,
}

#[derive(Component)]
pub struct GameCamera;

#[derive(Clone, Hash, Debug, PartialEq, Eq, SystemLabel)]
pub enum GameLabel {
    CameraScale,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 640.,
            height: 480.,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Game::default())
        .insert_resource(Difficulty::Normal)
        .add_event::<GameReset>()
        .add_event::<GameResetSend>()
        .add_state(GameState::Loading)
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(CommonPlugin)
        .add_plugin(LoadingPlugin)
        .add_plugin(BoardPlugin)
        .add_plugin(InfoScreenPlugin)
        .add_plugin(MiniGamePlugin)
        .add_plugin(IntroPlugin)
        .add_plugin(SetupPlugin)
        .add_startup_system(init)
        .add_system(camera_scale.label(GameLabel::CameraScale))
        .add_system(start_game)
        .add_system(reset_game)
        .add_system(reset_game_hotkey)
        .run();
}

pub fn init(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(GameCamera);
    commands.spawn_bundle(UiCameraBundle::default());
}

pub fn camera_scale(mut camera_query: Query<&mut Transform, With<GameCamera>>) {
    for mut transform in camera_query.iter_mut() {
        transform.scale = Vec3::new(0.5, 0.5, 1.0);
    }
}

pub fn start_game(
    mut game: ResMut<Game>,
    mut asset_library_ready: EventReader<AssetLibraryReady>,
    mut game_state: ResMut<State<GameState>>,
    difficulty: Res<Difficulty>,
) {
    for _ in asset_library_ready.iter() {
        match difficulty.as_ref() {
            Difficulty::Easy => {
                game.my_coins = 15;
                game.your_coins = 20;
            }
            Difficulty::Normal => {
                game.my_coins = 15;
                game.your_coins = 15;
            }
            Difficulty::Hard => {
                game.my_coins = 15;
                game.your_coins = 10;
            }
            Difficulty::VeryHard => {
                game.my_coins = 15;
                game.your_coins = 4;
            }
        }
        game.my_color = Color::rgba(1., 0.7, 0.7, 1.0);
        game.your_color = Color::YELLOW;
        game.turn = 1;
        game.duel = false;
        game.dice_roll = false;
        game_state.set(GameState::Setup).unwrap();
    }
}

pub fn reset_game(
    mut input: ResMut<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
    mut asset_library_ready: EventWriter<AssetLibraryReady>,
    entities: Query<Entity, Without<Camera>>,
    mut commands: Commands,
    mut game: ResMut<Game>,
    time: Res<Time>,
    mut reset_event: EventWriter<GameReset>,
    mut reset_send_event: EventReader<GameResetSend>,
) {
    let mut do_reset = false;
    for _ in reset_send_event.iter() {
        do_reset = true;
    }
    if !matches!(game_state.current(), GameState::Loading) {
        if do_reset {
            game_state.set(GameState::Loading).unwrap();
            input.reset(KeyCode::Key0);
            game.reset_timer = 1.;
            reset_event.send(GameReset);
            for entity in entities.iter() {
                commands.entity(entity).despawn();
            }
        }
    }
    if game.reset_timer > 0. {
        game.reset_timer -= time.delta_seconds();
        if game.reset_timer <= 0. {
            asset_library_ready.send(AssetLibraryReady);
            game.reset_timer = 0.;
        }
    }
}

pub fn reset_game_hotkey(
    mut input: ResMut<Input<KeyCode>>,
    game_state: Res<State<GameState>>,
    mut reset_event: EventWriter<GameResetSend>,
) {
    if !matches!(game_state.current(), GameState::Loading) {
        if input.just_pressed(KeyCode::Key0) {
            reset_event.send(GameResetSend);
            input.reset(KeyCode::Key0);
        }
    }
}

pub mod board;
pub mod common;
pub mod info_screen;
pub mod intro;
pub mod loading;
pub mod mini_game;
pub mod setup;

pub mod prelude {
    pub use super::{
        board::prelude::*, common::prelude::*, mini_game::prelude::*, Difficulty, Game, GameCamera,
        GameLabel, GameReset, GameResetSend, GameState,
    };
}
