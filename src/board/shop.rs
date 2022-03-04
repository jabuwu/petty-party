use super::item::RAPIER_COST;
use crate::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::Audio;

pub struct ShopOpen;

#[derive(Component)]
pub struct Shop;

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ShopOpen>()
            .add_system(open.label("shop_open"))
            .add_system(update.after("shop_open"));
    }
}

pub fn open(
    mut board: ResMut<Board>,
    mut shop_open: EventReader<ShopOpen>,
    mut commands: Commands,
    mut dialogue: ResMut<Dialogue>,
    asset_library: Res<AssetLibrary>,
) {
    for _ in shop_open.iter() {
        if board.first_shop {
            dialogue.add(DialogueEntry {
                text: "You made it to the shop!".into(),
                ..Default::default()
            });
            dialogue.add(DialogueEntry {
                text: "Here you can buy a rapier, which can be used to start a duel!".into(),
                ..Default::default()
            });
            dialogue.add(DialogueEntry {
                text: "During a duel, you can steal coins from your opponent.".into(),
                ..Default::default()
            });
            dialogue.add(DialogueEntry {
                text: "You can use items at the start of your turn.".into(),
                ..Default::default()
            });
            dialogue.add(DialogueEntry {
                text: "There is an indicator next to your coins when you have an item.".into(),
                ..Default::default()
            });
            dialogue.add(DialogueEntry {
                text: "As you can see, I'm already holding an item!".into(),
                ..Default::default()
            });
            board.first_shop = false;
        } else if !matches!(board.your_item, Item::None) {
            dialogue.add(DialogueEntry {
                text: "You already have an item!".into(),
                ..Default::default()
            });
        }
        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(50.0)),
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        bottom: Val::Px(0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                color: Color::NONE.into(),
                ..Default::default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    style: Style {
                        align_self: AlignSelf::Center,
                        ..Default::default()
                    },
                    text: Text::with_section(
                        "SPACE - Leave Shop\nR - Buy Rapier (10 coins)",
                        TextStyle {
                            font: asset_library.font("game"),
                            font_size: 24.0,
                            color: Color::BLACK,
                        },
                        TextAlignment {
                            horizontal: HorizontalAlign::Center,
                            vertical: VerticalAlign::Center,
                        },
                    ),
                    ..Default::default()
                });
            })
            .insert(Shop);
    }
}

pub fn update(
    mut dialogue: ResMut<Dialogue>,
    mut board: ResMut<Board>,
    mut game: ResMut<Game>,
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    shop_query: Query<Entity, With<Shop>>,
    audio: Res<Audio>,
    asset_library: Res<AssetLibrary>,
) {
    let mut shop_open = false;
    for _ in shop_query.iter() {
        shop_open = true;
    }
    if dialogue.busy() || !shop_open {
        return;
    }
    if !(matches!(board.your_item, Item::None)) {
        board.shop = false;
        for entity in shop_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        return;
    }
    if input.just_pressed(KeyCode::Space) {
        board.shop = false;
        for entity in shop_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
    } else if input.just_pressed(KeyCode::R) {
        if game.your_coins > RAPIER_COST + 3 {
            audio.play(asset_library.audio("itembuy"));
            board.your_item = Item::Rapier;
            board.your_item_use_interpolate = 0.;
            board.shop = false;
            for entity in shop_query.iter() {
                commands.entity(entity).despawn_recursive();
            }
            game.your_coins -= RAPIER_COST;
            dialogue.add(DialogueEntry {
                text: "You bought a rapier!".into(),
                ..Default::default()
            });
        } else if game.your_coins >= RAPIER_COST {
            dialogue.add(DialogueEntry {
                text: "Sorry, but you're about to land on a red tile!".into(),
                ..Default::default()
            });
            dialogue.add(DialogueEntry {
                text: "If you buy this now, you will lose the game!".into(),
                ..Default::default()
            });
        } else {
            dialogue.add(DialogueEntry {
                text: "Sorry! You can't afford it!".into(),
                ..Default::default()
            });
        }
    }
}
