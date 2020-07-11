use super::*;
use common::timer::TagType;

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

    give_player_level(Smelting, 10, &mut player);

    give_player_spawned_items(Ore, "Tin Ore", 4, &mut player, &mut inventories, &mut items);
    give_player_spawned_items(
        Material,
        "Softwood Log",
        1,
        &mut player,
        &mut inventories,
        &mut items,
    );

    // set facing to avoid change facing update
    player.facing = Direction::Right;

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

    give_player_level(Smelting, 45, &mut player);

    give_player_spawned_items(Ore, "Tin Ore", 4, &mut player, &mut inventories, &mut items);
    give_player_spawned_items(
        Material,
        "Softwood Log",
        1,
        &mut player,
        &mut inventories,
        &mut items,
    );

    // set facing to avoid change facing update
    player.facing = Direction::Right;

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

    let facility_id = get_facility_id_at(10, 5, &map);

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
        TagType::Duration(chrono::Duration::seconds(16))
    );

    assert_activity_started(16_000, ui::pane::PaneTitle::Smelting, &mut update_rx);

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
    assert_xp_is_updated(player.id, Smelting, 5, &mut update_rx);
    assert_activity_started(16_000, ui::pane::PaneTitle::Smelting, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(1, Material, "Tin Bar", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn can_smelt_cinnabar_to_mercury() {
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

    give_player_level(Smelting, 9, &mut player);

    give_player_spawned_items(
        Ore,
        "Cinnabar",
        4,
        &mut player,
        &mut inventories,
        &mut items,
    );
    give_player_spawned_items(
        Material,
        "Softwood Log",
        2,
        &mut player,
        &mut inventories,
        &mut items,
    );

    let facility_id = get_facility_id_at(10, 5, &map);

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
        &Command::ChoiceSelected(5, ActionContinuation::Smeltery, facility_id),
        Some(&update_tx),
        None,
    );

    assert!(activity.is_some());

    assert_eq!(
        timer.tags["ActivityComplete"],
        TagType::Duration(chrono::Duration::seconds(52))
    );

    assert_activity_started(52_000, ui::pane::PaneTitle::Smelting, &mut update_rx);

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
    assert_xp_is_updated(player.id, Smelting, 10, &mut update_rx);
    assert_activity_started(52_000, ui::pane::PaneTitle::Smelting, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(1, Material, "Mercury", &mut command_rx);
    assert_is_activity_abort(&mut command_rx);`
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}
#[test]
fn stops_when_supplies_run_out() {
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
        TagType::Duration(chrono::Duration::seconds(16))
    );

    assert_activity_started(16_000, ui::pane::PaneTitle::Smelting, &mut update_rx);

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
    assert_xp_is_updated(player.id, Smelting, 6, &mut update_rx);
    assert_activity_started(16_000, ui::pane::PaneTitle::Smelting, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(1, Material, "Copper Bar", &mut command_rx);
    assert_is_activity_abort(&mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn player_earns_5_xp_by_smelting_tin() {
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
        _update_tx,
        _update_rx,
        _command_tx,
        _command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(9, 5);

    give_player_level(Smelting, 45, &mut player);
    give_player_spawned_items(Ore, "Tin Ore", 4, &mut player, &mut inventories, &mut items);
    give_player_spawned_items(
        Material,
        "Softwood Log",
        1,
        &mut player,
        &mut inventories,
        &mut items,
    );
    let exp_xp = player.get_xp(Smelting) + 5;

    let facility_id = get_facility_id_at(10, 5, &map);

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
        None,
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
        None,
        None,
    );

    assert_eq!(player.get_xp(Smelting), exp_xp);
}

#[test]
fn player_earns_6_xp_by_smelting_copper() {
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
        _update_tx,
        _update_rx,
        _command_tx,
        _command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(9, 5);

    give_player_level(Smelting, 45, &mut player);
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
        "Softwood Log",
        1,
        &mut player,
        &mut inventories,
        &mut items,
    );
    let exp_xp = player.get_xp(Smelting) + 6;

    let facility_id = get_facility_id_at(10, 5, &map);

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
        None,
        None,
    );

    assert!(activity.is_some());

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
        None,
        None,
    );

    assert_eq!(player.get_xp(Smelting), exp_xp);
}
#[test]

fn player_earns_7_xp_by_smelting_bronze() {
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
        _update_tx,
        _update_rx,
        _command_tx,
        _command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(9, 5);

    give_player_level(Smelting, 4, &mut player);
    give_player_spawned_items(
        Ore,
        "Copper Ore",
        3,
        &mut player,
        &mut inventories,
        &mut items,
    );
    give_player_spawned_items(Ore, "Tin Ore", 1, &mut player, &mut inventories, &mut items);
    give_player_spawned_items(
        Material,
        "Softwood Log",
        1,
        &mut player,
        &mut inventories,
        &mut items,
    );
    let exp_xp = player.get_xp(Smelting) + 7;

    let facility_id = get_facility_id_at(10, 5, &map);

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
        &Command::ChoiceSelected(3, ActionContinuation::Smeltery, facility_id),
        None,
        None,
    );

    assert!(activity.is_some());

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
        None,
        None,
    );

    assert_eq!(player.get_xp(Smelting), exp_xp);
}

#[test]
fn player_earns_10_xp_by_smelting_cinnabar() {
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
        _update_tx,
        _update_rx,
        _command_tx,
        _command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(9, 5);

    give_player_level(Smelting, 9, &mut player);
    give_player_spawned_items(
        Ore,
        "Cinnabar",
        4,
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
    let exp_xp = player.get_xp(Smelting) + 10;

    let facility_id = get_facility_id_at(10, 5, &map);

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
        &Command::ChoiceSelected(5, ActionContinuation::Smeltery, facility_id),
        None,
        None,
    );

    assert!(activity.is_some());

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
        None,
        None,
    );

    assert_eq!(player.get_xp(Smelting), exp_xp);
}

#[test]
fn player_earns_75_xp_by_smelting_mythral() {
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
        _update_tx,
        _update_rx,
        _command_tx,
        _command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(9, 5);

    give_player_level(Smelting, 45, &mut player);
    give_player_spawned_items(
        Ore,
        "Mythral Ore",
        4,
        &mut player,
        &mut inventories,
        &mut items,
    );
    give_player_spawned_items(Ore, "Coal", 1, &mut player, &mut inventories, &mut items);
    let exp_xp = player.get_xp(Smelting) + 75;

    let facility_id = get_facility_id_at(10, 5, &map);

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
        &Command::ChoiceSelected(19, ActionContinuation::Smeltery, facility_id),
        None,
        None,
    );

    assert!(activity.is_some());

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
        None,
        None,
    );

    assert_eq!(player.get_xp(Smelting), exp_xp);
}
