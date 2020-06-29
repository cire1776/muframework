use super::*;

#[test]
fn can_saw_softwood_without_breaking_mill() {
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
    ) = initialize_game_system_with_player_at(12, 10);

    rng.set_succeed("lumbermill_breaks");

    let exp_xp = player.get_xp("construction") + 5;

    player.endorse_component_with(":wants_to_mill", "softwood");

    equip_player_with_spawned_item(
        Material,
        "Softwood Log",
        &mut player,
        &mut inventories,
        &mut items,
    );

    give_player_spawned_items(
        Material,
        "Softwood Log",
        3,
        &mut player,
        &mut inventories,
        &mut items,
    );

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

    assert!(activity.is_some());

    assert_eq!(
        timer.tags["ActivityComplete"],
        chrono::Duration::seconds(40)
    );

    assert_activity_started(40000, Sawing, &mut update_rx);

    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);

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
    assert_activity_started(40000, Sawing, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(player.id, Material, "Softwood Plank", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);

    assert_eq!(player.get_xp("construction"), exp_xp);
}

#[test]
fn can_saw_hardwood_without_breaking_mill() {
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
    ) = initialize_game_system_with_player_at(12, 10);

    rng.set_succeed("lumbermill_breaks");

    let exp_xp = player.get_xp("construction") + 10;

    player.endorse_component_with(":wants_to_mill", "hardwood");

    equip_player_with_spawned_item(
        Material,
        "Hardwood Log",
        &mut player,
        &mut inventories,
        &mut items,
    );

    give_player_spawned_items(
        Material,
        "Hardwood Log",
        3,
        &mut player,
        &mut inventories,
        &mut items,
    );

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

    assert!(activity.is_some());

    assert_eq!(
        timer.tags["ActivityComplete"],
        chrono::Duration::seconds(60)
    );

    assert_activity_started(60000, Sawing, &mut update_rx);

    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);

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
    assert_activity_started(60000, Sawing, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(player.id, Material, "Hardwood Plank", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);

    assert_eq!(player.get_xp("construction"), exp_xp);
}

#[test]
fn mills_can_break() {
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
    ) = initialize_game_system_with_player_at(12, 10);

    rng.set_fail("lumbermill_breaks");

    player.endorse_component_with(":wants_to_mill", "hardwood");

    equip_player_with_spawned_item(
        Material,
        "Hardwood Log",
        &mut player,
        &mut inventories,
        &mut items,
    );

    give_player_spawned_items(
        Material,
        "Hardwood Log",
        3,
        &mut player,
        &mut inventories,
        &mut items,
    );

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

    assert!(activity.is_some());

    assert_eq!(
        timer.tags["ActivityComplete"],
        chrono::Duration::seconds(60)
    );

    assert_activity_started(60_000, Sawing, &mut update_rx);

    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);

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
    assert_activity_started(60_000, Sawing, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(player.id, Material, "Hardwood Plank", &mut command_rx);
    assert_is_destroy_facility(&mut command_rx);
    assert_is_activity_abort(&mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}
