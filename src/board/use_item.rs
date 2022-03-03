use crate::prelude::*;
use bevy::prelude::*;

pub struct UseItem {
    time: f32,
    item: Item,
}

pub struct UseItemPlugin;

impl Plugin for UseItemPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UseItem {
            time: 0.0,
            item: Item::None,
        })
        .add_system_set(SystemSet::on_enter(BoardState::UseItem).with_system(init))
        .add_system_set(SystemSet::on_exit(BoardState::UseItem).with_system(cleanup))
        .add_system_set(SystemSet::on_update(BoardState::UseItem).with_system(update));
    }
}

pub fn init(
    mut game: ResMut<Game>,
    mut use_item: ResMut<UseItem>,
    board: Res<Board>,
    mut dialogue: ResMut<Dialogue>,
) {
    use_item.time = 0.;
    use_item.item = board.your_item;
    if matches!(use_item.item, Item::CrystalBall) {
        dialogue.add(DialogueEntry {
            text: "You used the crystal ball!".into(),
            ..Default::default()
        });
    } else if matches!(use_item.item, Item::Rapier) {
        game.duel = true;
        dialogue.add(DialogueEntry {
            text: "You used the rapier!".into(),
            ..Default::default()
        });
        dialogue.add(DialogueEntry {
            text: "The mini game will now be a duel.".into(),
            ..Default::default()
        });
    }
}

pub fn cleanup(mut board: ResMut<Board>, mut use_item: ResMut<UseItem>) {
    board.my_item_use_interpolate = 0.;
    board.your_item = Item::None;
    use_item.time = 0.;
    use_item.item = Item::None;
}

pub fn update(
    mut board: ResMut<Board>,
    mut board_state: ResMut<State<BoardState>>,
    dialogue: Res<Dialogue>,
    time: Res<Time>,
) {
    board.your_item_use_interpolate += time.delta_seconds();
    board.your_item_use_interpolate = board.your_item_use_interpolate.clamp(0., 1.);
    if !dialogue.busy() && board.your_item_use_interpolate == 1. {
        board_state.set(BoardState::TurnInput).unwrap();
    }
}
