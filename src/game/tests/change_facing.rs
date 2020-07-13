use super::*;
use Direction::*;

#[test]
fn changing_direction_to_the_current_direction_does_nothing() {
    let (
        mut player,
        mut map,
        mut obstacles,
        mut characters,
        mut item_class_specifiers,
        mut items,
        mut facilities,
        mut inventories,
        mut game_data,
        mut rng,
        mut timer,
        update_tx,
        mut update_rx,
        _command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(12, 10);

    player.facing = Up;

    let activity = game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut game_data,
        &mut rng,
        &mut timer,
        None,
        &Command::ChangeFacing(Up),
        Some(&update_tx),
        None,
    );

    assert!(activity.is_none());

    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn changing_direction_to_left_changes_the_player_and_announces_the_change() {
    let (
        mut player,
        mut map,
        mut obstacles,
        mut characters,
        mut item_class_specifiers,
        mut items,
        mut facilities,
        mut inventories,
        mut game_data,
        mut rng,
        mut timer,
        update_tx,
        mut update_rx,
        _command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(12, 10);

    player.facing = Up;

    let activity = game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut game_data,
        &mut rng,
        &mut timer,
        None,
        &Command::ChangeFacing(Left),
        Some(&update_tx),
        None,
    );

    assert!(activity.is_none());

    assert_eq!(player.facing, Left);

    assert_character_facing_changed(player.id, Left, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}
