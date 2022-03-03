use bevy::prelude::*;
use bevy_kira_audio::AudioSource;
use std::collections::HashMap;

// TODO: remove uses of asset server

pub struct LoadTime(f32);

pub struct AssetLibraryReady;

#[derive(Default)]
pub struct AssetLibrary {
    images: HashMap<String, Handle<Image>>,
    texture_atlases: HashMap<String, Handle<TextureAtlas>>,
    audio: HashMap<String, Handle<AudioSource>>,
}

impl AssetLibrary {
    pub fn image(&self, name: &str) -> Handle<Image> {
        self.images.get(name).unwrap().clone()
    }

    pub fn texture_atlas(&self, name: &str) -> Handle<TextureAtlas> {
        self.texture_atlases.get(name).unwrap().clone()
    }

    pub fn audio(&self, name: &str) -> Handle<AudioSource> {
        self.audio.get(name).unwrap().clone()
    }

    pub fn load_audio(&mut self, asset_server: &Res<AssetServer>) {}
}

pub struct AssetLibraryPlugin;

impl Plugin for AssetLibraryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AssetLibrary>()
            .insert_resource(LoadTime(0.))
            .add_event::<AssetLibraryReady>()
            .add_startup_system(init_assets)
            .add_system(load);
    }
}

pub fn init_assets(
    mut asset_library: ResMut<AssetLibrary>,
    asset_server: Res<AssetServer>,
    mut res_texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let images: Vec<(&str, &str)> = vec![
        ("tile_blue", "sprites/tile_blue.png"),
        ("tile_red", "sprites/tile_red.png"),
        ("tile_green", "sprites/tile_green.png"),
        ("bg", "sprites/bg.png"),
        ("score_overlay", "sprites/score_overlay.png"),
        ("boats_bg", "sprites/boats_bg.png"),
        ("boat", "sprites/boat.png"),
        ("cannon", "sprites/cannon.png"),
        ("cannon_shadow", "sprites/cannon_shadow.png"),
        ("rps", "sprites/rps.png"),
        ("item_rapier", "sprites/item_rapier.png"),
        ("item_mystery", "sprites/item_mystery.png"),
        ("static", "sprites/static.png"),
        ("me_1", "sprites/me_1.png"),
        ("me_2", "sprites/me_2.png"),
        ("heart", "sprites/heart.png"),
        ("heart_empty", "sprites/heart_empty.png"),
    ];
    let texture_atlases: Vec<(&str, &str, Vec2, usize, usize)> = vec![
        (
            "dice_roll",
            "sprites/dice_roll.png",
            Vec2::new(48.0, 48.0),
            15,
            1,
        ),
        ("rps", "sprites/rps.png", Vec2::new(48.0, 48.0), 3, 1),
        ("duel", "sprites/duel.png", Vec2::new(192.0, 192.0), 6, 1),
    ];
    let audios: Vec<(&str, &str)> = vec![
        //("music", "music/music.ogg"),
        ("dialogue", "sfx/dialogue.ogg"),
        ("diceroll", "sfx/diceroll.ogg"),
        ("diceding", "sfx/diceding.ogg"),
        ("move", "sfx/move.ogg"),
        ("turnstart", "sfx/turnstart.ogg"),
        ("ready", "sfx/ready.ogg"),
        ("start", "sfx/start.ogg"),
        ("finish", "sfx/finish.ogg"),
        ("cannon", "sfx/cannon.ogg"),
        ("waterdrop", "sfx/waterdrop.ogg"),
        ("boathit", "sfx/boathit.ogg"),
        ("rock", "sfx/rock.ogg"),
        ("paper", "sfx/paper.ogg"),
        ("scissors", "sfx/scissors.ogg"),
        ("shoot", "sfx/shoot.ogg"),
        ("duelhit", "sfx/duelhit.ogg"),
        ("duelblock", "sfx/duelblock.ogg"),
        ("static", "sfx/static.ogg"),
    ];

    for image_def in images.iter() {
        asset_library
            .images
            .insert(image_def.0.into(), asset_server.load(image_def.1));
    }

    for texture_atlas_def in texture_atlases.iter() {
        let texture_image: Handle<Image> = asset_server.load(texture_atlas_def.1);
        let texture_atlas = TextureAtlas::from_grid(
            texture_image,
            texture_atlas_def.2,
            texture_atlas_def.3,
            texture_atlas_def.4,
        );
        asset_library.texture_atlases.insert(
            texture_atlas_def.0.into(),
            res_texture_atlases.add(texture_atlas),
        );
    }
    for audio_def in audios.iter() {
        asset_library
            .audio
            .insert(audio_def.0.into(), asset_server.load(audio_def.1));
    }
}

pub fn load(
    mut load_time: ResMut<LoadTime>,
    time: Res<Time>,
    mut asset_library_ready: EventWriter<AssetLibraryReady>,
) {
    if load_time.0 < 1. {
        load_time.0 += time.delta_seconds();
        if load_time.0 >= 1. {
            asset_library_ready.send(AssetLibraryReady);
        }
    }
}
