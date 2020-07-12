use super::*;

use common::timer::TagType;
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

    rng.set_fail("levelling check");

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
        TagType::Duration(chrono::Duration::seconds(57))
    );
    assert!(activity.is_some());

    assert_activity_started(57_000, PaneTitle::Cooking, &mut update_rx);
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
    assert_xp_is_updated(player.id, Cooking, exp_xp, &mut update_rx);
    assert_activity_started(57000, PaneTitle::Cooking, &mut update_rx);
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
        TagType::Duration(chrono::Duration::seconds(60))
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
    assert_xp_is_updated(player.id, Cooking, exp_xp, &mut update_rx);
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
fn can_cook_item_at_ready() {
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

    let exp_shrimp_count = 0;
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
        TagType::Duration(chrono::Duration::seconds(57))
    );
    assert!(activity.is_some());

    assert_activity_started(57_000, PaneTitle::Cooking, &mut update_rx);

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
    assert_xp_is_updated(player.id, Cooking, exp_xp, &mut update_rx);
    assert_activity_started(57_000, PaneTitle::Cooking, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(player.id, Food, "Grilled Shrimp", &mut command_rx);
    assert_is_activity_abort(&mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);

    let inventory = inventories
        .get(&player.inventory_id())
        .expect("unable to get inventory made long.")
        .clone();

    let shrimp_count = inventory.count_of(Ingredient, "Shrimp");
    let log_count = inventory.count_of(Material, "Softwood Log");

    let mounted_item_id = player.mounting_points.at(&MountingPoint::AtReady);

    assert!(mounted_item_id.is_none());

    assert_eq!(shrimp_count, exp_shrimp_count);
    assert_eq!(log_count, exp_log_count);

    assert_eq!(player.get_xp(Cooking), exp_xp);
}

#[test]
fn timer_is_reduced_by_skill_time() {
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
    give_player_level(Cooking, 10, &mut player);

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
        TagType::Duration(chrono::Duration::seconds(51))
    );
    assert!(activity.is_some());

    assert_activity_started(51_000, PaneTitle::Cooking, &mut update_rx);

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
    assert_xp_is_updated(player.id, Cooking, 3, &mut update_rx);
    assert_activity_started(51_000, PaneTitle::Cooking, &mut update_rx);
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
}

#[test]
fn regression_divide_by_zero_crash_when_cooking_at_required_level() {
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
        _update_rx,
        command_tx,
        _command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(11, 6);

    rng.set_succeed("cooking_success");

    player.endorse_component_with(":wants_to_cook", "shrimp");
    give_player_level(Cooking, 1, &mut player);

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
}

#[test]
fn regression_endorsement_left_on_after_unequip() {
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
        _update_rx,
        command_tx,
        _command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(11, 6);

    rng.set_succeed("cooking_success");

    player.endorse_component_with(":wants_to_cook", "shrimp");
    give_player_level(Cooking, 1, &mut player);

    equip_player_with_spawned_item(
        Ingredient,
        "Shrimp",
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
        Some(&update_tx),
        None,
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

    assert!(!player.is_endorsed_with(":wants_to_cook"))
}
