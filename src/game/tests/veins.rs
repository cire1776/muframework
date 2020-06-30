use super::*;
use ui::PaneTitle;

#[test]
fn can_dig_dirt_without_exhaustion() {
    let (
        mut player,
        mut map,
        mut obstacles,
        mut characters,
        mut item_class_specifiers,
        mut items,
        mut facilities,
        mut inventories,
        mut rng,
        mut timer,
        update_tx,
        mut update_rx,
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(11, 7);

    rng.set_fail("chance_of_exhaustion");

    player.endorse_with(":can_dig");

    let activity = game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Down, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_eq!(
        timer.tags["ActivityComplete"],
        chrono::Duration::seconds(40)
    );

    assert_activity_started(40_000, Digging, &mut update_rx);

    game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut rng,
        &mut timer,
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert_activity_expired(&mut update_rx);
    assert_activity_started(40_000, Digging, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(player.id, Ore, "Dirt", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn can_dig_dirt_with_exhaustion() {
    let (
        mut player,
        mut map,
        mut obstacles,
        mut characters,
        mut item_class_specifiers,
        mut items,
        mut facilities,
        mut inventories,
        mut rng,
        mut timer,
        update_tx,
        mut update_rx,
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(11, 7);

    rng.set_succeed("chance_of_exhaustion");

    player.endorse_with(":can_dig");

    let activity = game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Down, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_eq!(
        timer.tags["ActivityComplete"],
        chrono::Duration::seconds(40)
    );

    assert_activity_started(40_000, Digging, &mut update_rx);

    game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut rng,
        &mut timer,
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert_activity_expired(&mut update_rx);
    assert_activity_started(40_000, Digging, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(player.id, Ore, "Dirt", &mut command_rx);

    assert_is_destroy_facility(&mut command_rx);
    assert_is_activity_abort(&mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn can_dig_sand_without_exhaustion() {
    let (
        mut player,
        mut map,
        mut obstacles,
        mut characters,
        mut item_class_specifiers,
        mut items,
        mut facilities,
        mut inventories,
        mut rng,
        mut timer,
        update_tx,
        mut update_rx,
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(12, 7);

    rng.set_fail("chance_of_exhaustion");

    player.endorse_with(":can_dig");

    let activity = game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Down, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_eq!(
        timer.tags["ActivityComplete"],
        chrono::Duration::seconds(20)
    );

    assert_activity_started(20_000, Digging, &mut update_rx);

    game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut rng,
        &mut timer,
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert_activity_expired(&mut update_rx);
    assert_activity_started(20_000, Digging, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(player.id, Ore, "Sand", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn can_dig_sand_with_exhaustion() {
    let (
        mut player,
        mut map,
        mut obstacles,
        mut characters,
        mut item_class_specifiers,
        mut items,
        mut facilities,
        mut inventories,
        mut rng,
        mut timer,
        update_tx,
        mut update_rx,
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(12, 7);

    rng.set_succeed("chance_of_exhaustion");

    player.endorse_with(":can_dig");

    let activity = game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Down, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_eq!(
        timer.tags["ActivityComplete"],
        chrono::Duration::seconds(20)
    );

    assert_activity_started(20_000, Digging, &mut update_rx);

    game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut rng,
        &mut timer,
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert_activity_expired(&mut update_rx);
    assert_activity_started(20_000, Digging, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(player.id, Ore, "Sand", &mut command_rx);
    assert_is_destroy_facility(&mut command_rx);
    assert_is_activity_abort(&mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn can_mine_stone_without_exhaustion() {
    let (
        mut player,
        mut map,
        mut obstacles,
        mut characters,
        mut item_class_specifiers,
        mut items,
        mut facilities,
        mut inventories,
        mut rng,
        mut timer,
        update_tx,
        mut update_rx,
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(13, 7);

    rng.set_fail("chance_of_exhaustion");

    player.endorse_with(":can_mine");

    let activity = game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Down, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_eq!(
        timer.tags["ActivityComplete"],
        chrono::Duration::seconds(60)
    );

    assert_activity_started(60_000, PaneTitle::Mining, &mut update_rx);

    game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut rng,
        &mut timer,
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert_activity_expired(&mut update_rx);
    assert_activity_started(60_000, PaneTitle::Mining, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(player.id, Ore, "Stone", &mut command_rx);

    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn can_mine_stone_with_exhaustion() {
    let (
        mut player,
        mut map,
        mut obstacles,
        mut characters,
        mut item_class_specifiers,
        mut items,
        mut facilities,
        mut inventories,
        mut rng,
        mut timer,
        update_tx,
        mut update_rx,
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(13, 7);

    rng.set_succeed("chance_of_exhaustion");

    player.endorse_with(":can_mine");

    let activity = game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Down, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_eq!(
        timer.tags["ActivityComplete"],
        chrono::Duration::seconds(60)
    );

    assert_activity_started(60_000, PaneTitle::Mining, &mut update_rx);

    game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut rng,
        &mut timer,
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert_activity_expired(&mut update_rx);
    assert_activity_started(60_000, PaneTitle::Mining, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(player.id, Ore, "Stone", &mut command_rx);

    assert_is_destroy_facility(&mut command_rx);
    assert_is_activity_abort(&mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn can_mine_tin_without_exhaustion() {
    let (
        mut player,
        mut map,
        mut obstacles,
        mut characters,
        mut item_class_specifiers,
        mut items,
        mut facilities,
        mut inventories,
        mut rng,
        mut timer,
        update_tx,
        mut update_rx,
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(14, 7);

    rng.set_fail("chance_of_exhaustion");

    player.endorse_with(":can_mine");

    let activity = game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Down, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_eq!(
        timer.tags["ActivityComplete"],
        chrono::Duration::seconds(60)
    );

    assert_activity_started(60_000, PaneTitle::Mining, &mut update_rx);

    game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut rng,
        &mut timer,
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert_activity_expired(&mut update_rx);
    assert_activity_started(60_000, PaneTitle::Mining, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(player.id, Ore, "Tin Ore", &mut command_rx);

    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn can_mine_tin_with_exhaustion() {
    let (
        mut player,
        mut map,
        mut obstacles,
        mut characters,
        mut item_class_specifiers,
        mut items,
        mut facilities,
        mut inventories,
        mut rng,
        mut timer,
        update_tx,
        mut update_rx,
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(14, 7);

    rng.set_succeed("chance_of_exhaustion");

    player.endorse_with(":can_mine");

    let activity = game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Down, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_eq!(
        timer.tags["ActivityComplete"],
        chrono::Duration::seconds(60)
    );

    assert_activity_started(60_000, PaneTitle::Mining, &mut update_rx);

    game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut rng,
        &mut timer,
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert_activity_expired(&mut update_rx);
    assert_activity_started(60_000, PaneTitle::Mining, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(player.id, Ore, "Tin Ore", &mut command_rx);

    assert_is_destroy_facility(&mut command_rx);
    assert_is_activity_abort(&mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn can_mine_copper_without_exhaustion() {
    let (
        mut player,
        mut map,
        mut obstacles,
        mut characters,
        mut item_class_specifiers,
        mut items,
        mut facilities,
        mut inventories,
        mut rng,
        mut timer,
        update_tx,
        mut update_rx,
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(15, 7);

    rng.set_fail("chance_of_exhaustion");

    player.endorse_with(":can_mine");

    let activity = game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Down, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_eq!(
        timer.tags["ActivityComplete"],
        chrono::Duration::seconds(60)
    );

    assert_activity_started(60_000, PaneTitle::Mining, &mut update_rx);

    game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut rng,
        &mut timer,
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert_activity_expired(&mut update_rx);
    assert_activity_started(60_000, PaneTitle::Mining, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(player.id, Ore, "Copper Ore", &mut command_rx);

    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn can_mine_copper_with_exhaustion() {
    let (
        mut player,
        mut map,
        mut obstacles,
        mut characters,
        mut item_class_specifiers,
        mut items,
        mut facilities,
        mut inventories,
        mut rng,
        mut timer,
        update_tx,
        mut update_rx,
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(15, 7);

    rng.set_succeed("chance_of_exhaustion");

    player.endorse_with(":can_mine");

    let activity = game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Down, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_eq!(
        timer.tags["ActivityComplete"],
        chrono::Duration::seconds(60)
    );

    assert_activity_started(60_000, PaneTitle::Mining, &mut update_rx);

    game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut rng,
        &mut timer,
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert_activity_expired(&mut update_rx);
    assert_activity_started(60_000, PaneTitle::Mining, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(player.id, Ore, "Copper Ore", &mut command_rx);
    assert_is_destroy_facility(&mut command_rx);
    assert_is_activity_abort(&mut command_rx);

    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}
