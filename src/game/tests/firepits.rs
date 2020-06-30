use super::*;

use ui::PaneTitle;

#[test]
fn can_cook_fish_successfully() {
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
    ) = initialize_game_system_with_player_at(11, 6);

    rng.set_succeed("cooking_success");

    player.endorse_component_with(":wants_to_cook", "shrimp");
    give_player_level(Cooking, 4, &mut player);

    let exp_xp = player.get_xp(Cooking) + 3;

    equip_player_with_spawned_item(
        Ingredient,
        "Shrimp",
        &mut player,
        &mut inventories,
        &mut items,
    );

    give_player_spawned_items(
        Ingredient,
        "Shrimp",
        2,
        &mut player,
        &mut inventories,
        &mut items,
    );

    give_player_spawned_items(
        Material,
        "Softwood Log",
        1,
        &mut player,
        &mut inventories,
        &mut items,
    );
    let inventory = inventories
        .get(&player.inventory_id())
        .expect("unable to get inventory made long.")
        .clone();

    let exp_shrimp_count = inventory.count_of(Ingredient, "Shrimp") - 1;
    let exp_log_count = inventory.count_of(Material, "Softwood Log") - 1;

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
        &Command::Move(Direction::Up, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_player_is_at(11, 6, &player);

    assert_eq!(
        timer.tags["ActivityComplete"],
        chrono::Duration::seconds(60)
    );
    assert!(activity.is_some());

    assert_activity_started(60_000, PaneTitle::Cooking, &mut update_rx);

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
    assert_activity_started(60000, PaneTitle::Cooking, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(player.id, Food, "Grilled Shrimp", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);

    let inventory = inventories
        .get(&player.inventory_id())
        .expect("unable to get inventory made long.")
        .clone();

    let shrimp_count = inventory.count_of(Ingredient, "Shrimp");
    let log_count = inventory.count_of(Material, "Softwood Log");

    assert_eq!(shrimp_count, exp_shrimp_count);
    assert_eq!(log_count, exp_log_count);

    assert_eq!(player.get_xp(Cooking), exp_xp);
}

#[test]
fn can_burn_fish() {
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
    ) = initialize_game_system_with_player_at(11, 6);

    rng.set_fail("cooking_success");

    player.endorse_component_with(":wants_to_cook", "shrimp");

    let exp_xp = player.get_xp(Cooking) + 1;

    equip_player_with_spawned_item(
        Ingredient,
        "Shrimp",
        &mut player,
        &mut inventories,
        &mut items,
    );

    give_player_spawned_items(
        Ingredient,
        "Shrimp",
        2,
        &mut player,
        &mut inventories,
        &mut items,
    );

    give_player_spawned_items(
        Material,
        "Softwood Log",
        1,
        &mut player,
        &mut inventories,
        &mut items,
    );
    let inventory = inventories
        .get(&player.inventory_id())
        .expect("unable to get inventory made long.")
        .clone();

    let exp_shrimp_count = inventory.count_of(Ingredient, "Shrimp") - 1;
    let exp_log_count = inventory.count_of(Material, "Softwood Log") - 1;

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
        &Command::Move(Direction::Up, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_player_is_at(11, 6, &player);

    assert_eq!(
        timer.tags["ActivityComplete"],
        chrono::Duration::seconds(60)
    );
    assert!(activity.is_some());

    assert_activity_started(60000, PaneTitle::Cooking, &mut update_rx);

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
    assert_activity_started(60000, PaneTitle::Cooking, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(player.id, Material, "Burnt Shrimp", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);

    let inventory = inventories
        .get(&player.inventory_id())
        .expect("unable to get inventory made long.")
        .clone();

    let shrimp_count = inventory.count_of(Ingredient, "Shrimp");
    let log_count = inventory.count_of(Material, "Softwood Log");

    assert_eq!(shrimp_count, exp_shrimp_count);
    assert_eq!(log_count, exp_log_count);

    assert_eq!(player.get_xp(Cooking), exp_xp);
}

#[test]
fn stops_cooking_when_supplies_run_out() {
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
    ) = initialize_game_system_with_player_at(11, 6);

    rng.set_succeed("cooking_success");

    player.endorse_component_with(":wants_to_cook", "shrimp");
    give_player_level(Cooking, 4, &mut player);

    equip_player_with_spawned_item(
        Ingredient,
        "Shrimp",
        &mut player,
        &mut inventories,
        &mut items,
    );

    give_player_spawned_items(
        Ingredient,
        "Shrimp",
        1,
        &mut player,
        &mut inventories,
        &mut items,
    );

    give_player_spawned_items(
        Material,
        "Softwood Log",
        1,
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
        &Command::Move(Direction::Up, MoveCommandMode::Use),
        None,
        None,
    );

    assert_eq!(
        timer.tags["ActivityComplete"],
        chrono::Duration::seconds(60)
    );
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
    assert_activity_started(60_000, PaneTitle::Cooking, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(player.id, Food, "Grilled Shrimp", &mut command_rx);
    assert_is_activity_abort(&mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}
