use super::*;

#[test]
fn can_open_a_smeltery_at_level_10() {
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
        _command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(9, 5);

    give_player_level("smelting", 10, &mut player);

    give_player_spawned_items(Ore, "Tin Ore", 4, &mut player, &mut inventories, &mut items);
    give_player_spawned_items(
        Material,
        "Softwood Log",
        1,
        &mut player,
        &mut inventories,
        &mut items,
    );

    let _activity = game_state.game_loop_iteration(
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
        &Command::Move(Direction::Right, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_is_display_options(
        vec!["Tin", "Copper", "Bronze", "Lead", "Mercury"],
        ActionContinuation::Smeltery,
        &mut update_rx,
    );
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn can_open_a_smeltery_at_level_45() {
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
        _command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(9, 5);

    give_player_level("smelting", 45, &mut player);

    give_player_spawned_items(Ore, "Tin Ore", 4, &mut player, &mut inventories, &mut items);
    give_player_spawned_items(
        Material,
        "Softwood Log",
        1,
        &mut player,
        &mut inventories,
        &mut items,
    );

    let _activity = game_state.game_loop_iteration(
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
        &Command::Move(Direction::Right, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_is_display_options(
        vec![
            "Tin",
            "Copper",
            "Bronze",
            "Lead",
            "Mercury",
            "Iron",
            "Tungsten",
            "Cobalt",
            "Nickel",
            "Steel",
            "Gold",
            "Aluminum",
            "Silver",
            "Zinc",
            "Platinum",
            "Stainless Steel",
            "Stellite",
            "Titanium",
            "Mythral",
        ],
        ActionContinuation::Smeltery,
        &mut update_rx,
    );
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn can_smelt_tin_with_excess_quantities() {
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
    ) = initialize_game_system_with_player_at(9, 5);

    give_player_spawned_items(Ore, "Tin Ore", 8, &mut player, &mut inventories, &mut items);
    give_player_spawned_items(
        Material,
        "Softwood Log",
        2,
        &mut player,
        &mut inventories,
        &mut items,
    );

    let facility_id = match map.at(10, 5) {
        tile_map::Tile::Facility(id) => id,
        _ => panic!("smeltery not found"),
    };

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
        &Command::ChoiceSelected(1, ActionContinuation::Smeltery, facility_id),
        Some(&update_tx),
        None,
    );

    assert!(activity.is_some());

    assert_eq!(
        timer.tags["ActivityComplete"],
        chrono::Duration::seconds(15)
    );

    assert_activity_started(15_000, ui::pane::PaneTitle::Smelting, &mut update_rx);

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
    assert_activity_started(15_000, ui::pane::PaneTitle::Smelting, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(1, Material, "Tin Bar", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn can_smelt_tin_with_just_enough_for_one() {
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
    ) = initialize_game_system_with_player_at(9, 5);

    give_player_spawned_items(
        Ore,
        "Copper Ore",
        4,
        &mut player,
        &mut inventories,
        &mut items,
    );
    give_player_spawned_items(
        Material,
        "Hardwood Log",
        1,
        &mut player,
        &mut inventories,
        &mut items,
    );

    let facility_id = match map.at(10, 5) {
        tile_map::Tile::Facility(id) => id,
        _ => panic!("smeltery not found"),
    };

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
        &Command::ChoiceSelected(2, ActionContinuation::Smeltery, facility_id),
        Some(&update_tx),
        None,
    );

    assert!(activity.is_some());

    assert_eq!(
        timer.tags["ActivityComplete"],
        chrono::Duration::seconds(15)
    );

    assert_activity_started(15_000, ui::pane::PaneTitle::Smelting, &mut update_rx);

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
    assert_activity_started(15_000, ui::pane::PaneTitle::Smelting, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(1, Material, "Copper Bar", &mut command_rx);
    assert_is_activity_abort(&mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}
