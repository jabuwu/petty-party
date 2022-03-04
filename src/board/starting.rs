use crate::prelude::*;
use bevy::prelude::*;

#[derive(Default)]
pub struct Starting {
    time: f32,
}

#[derive(Component)]
pub struct StartingPan;

pub struct StartingPlugin;

impl Plugin for StartingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Starting { time: 0.0 })
            .add_system_set(SystemSet::on_enter(BoardState::Starting).with_system(init))
            .add_system_set(SystemSet::on_exit(BoardState::Starting).with_system(cleanup))
            .add_system_set(SystemSet::on_update(BoardState::Starting).with_system(update));
    }
}

pub fn init(
    mut commands: Commands,
    mut starting: ResMut<Starting>,
    mut camera_controller: ResMut<CameraController>,
    mut dialogue: ResMut<Dialogue>,
    game: Res<Game>,
    mut board: ResMut<Board>,
) {
    let follow_entity = commands
        .spawn()
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(StartingPan)
        .id();
    starting.time = 0.;
    camera_controller.zoom_out = true;
    camera_controller.follow_entity = Some(follow_entity);
    if game.turn == 1 {
        dialogue.add(DialogueEntry {
            text: "Welcome to my board game!".into(),
            ..Default::default()
        });
        dialogue.add(DialogueEntry {
            text: "You roll the dice and move that many tiles!".into(),
            ..Default::default()
        });
        dialogue.add(DialogueEntry {
            text: "If you land on a blue tile, you get 3 coins!".into(),
            ..Default::default()
        });
        dialogue.add(DialogueEntry {
            text: "If you land on a red tile, you lose 3 coins!".into(),
            ..Default::default()
        });
        dialogue.add(DialogueEntry {
            text: "If you pass a green tile, you can buy items!".into(),
            ..Default::default()
        });
        dialogue.add(DialogueEntry {
            text: "After your turn, we play a mini game!".into(),
            ..Default::default()
        });
        dialogue.add(DialogueEntry {
            text: "The winner of the mini game gets more coins.".into(),
            ..Default::default()
        });
        dialogue.add(DialogueEntry {
            text: "Huh? Mario? No, I don't know anyone by that name...".into(),
            ..Default::default()
        });
        dialogue.add(DialogueEntry {
            text: "Let's begin!!".into(),
            ..Default::default()
        });
    } else if matches!(board.your_item, Item::Rapier) && board.rapier_dialog {
        dialogue.add(DialogueEntry {
            text: "Remember: Whoever runs out of coins first loses!".into(),
            ..Default::default()
        });
        dialogue.add(DialogueEntry {
            text: "I keep landing on blue, so I don't think I will run out...".into(),
            ..Default::default()
        });
        dialogue.add(DialogueEntry {
            text: "Unless of course you use that rapier to start a duel!".into(),
            ..Default::default()
        });
        dialogue.add(DialogueEntry {
            text: "But remember, I'm not the only one that can lose coins that way!".into(),
            ..Default::default()
        });
        board.rapier_dialog = false;
    }
}

pub fn cleanup(mut starting: ResMut<Starting>, mut camera_controller: ResMut<CameraController>) {
    starting.time = 0.;
    camera_controller.zoom_out = false;
}

pub fn update(
    game: Res<Game>,
    mut starting: ResMut<Starting>,
    time: Res<Time>,
    mut board_state: ResMut<State<BoardState>>,
    dialogue: Res<Dialogue>,
    mut pan_query: Query<&mut Transform, With<StartingPan>>,
) {
    if game.my_coins == 0 || game.your_coins == 0 {
        board_state.set(BoardState::EndGame).unwrap();
        return;
    }
    starting.time += time.delta_seconds();
    if !dialogue.busy() {
        board_state.set(BoardState::TurnIntro).unwrap();
    }
    for mut pan in pan_query.iter_mut() {
        let amt = -starting.time * 0.1;
        pan.translation.x = amt.cos() * 90.;
        pan.translation.y = amt.sin() * 150.;
    }
}
