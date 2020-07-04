use super::*;
use common::timer::TagType;

#[test]
fn cannot_pick_from_patch_without_endorcement_can_pick() {
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
        _command_tx,
        _command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(1, 2);

    player.unendorse_with(":can_pick");

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

    assert_player_is_at(1, 2, &player);
    assert!(activity.is_none());
}

#[test]
fn picking_can_done_with_can_pick_endorsement() {
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
    ) = initialize_game_system_with_player_at(1, 2);

    player.endorse_with(":can_pick");

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

    assert!(activity.is_some());

    assert_eq!(
        timer.tags["ActivityComplete"],
        TagType::Duration(chrono::Duration::seconds(60))
    );

    assert_activity_started(60_000, ui::PaneTitle::Harvesting, &mut update_rx);

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
    assert_activity_started(60000, ui::PaneTitle::Harvesting, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(player.inventory_id(), Food, "Radish", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn can_produce_multiple_types_of_produce() {
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
    ) = initialize_game_system_with_player_at(1, 2);

    player.endorse_with(":can_pick");

    let patch_id = get_facility_id_at(1, 1, &map);
    let patch = facilities
        .get_mut(patch_id)
        .expect("unable to locate patch");

    patch.set_property("produce", 6);

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

    assert!(activity.is_some());

    assert_eq!(
        timer.tags["ActivityComplete"],
        TagType::Duration(chrono::Duration::seconds(60))
    );

    assert_activity_started(60_000, ui::PaneTitle::Harvesting, &mut update_rx);

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
    assert_activity_started(60000, ui::PaneTitle::Harvesting, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(player.inventory_id(), Food, "Carrot", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn takes_skilltime_into_account() {
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
    ) = initialize_game_system_with_player_at(1, 2);

    give_player_level(Harvesting, 10, &mut player);
    player.endorse_with(":can_pick");

    let patch_id = get_facility_id_at(1, 1, &map);
    let patch = facilities
        .get_mut(patch_id)
        .expect("unable to locate patch");

    patch.set_property("produce", 21);

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

    assert!(activity.is_some());

    assert_eq!(
        timer.tags["ActivityComplete"],
        TagType::Duration(chrono::Duration::seconds(51))
    );

    assert_activity_started(51_000, ui::PaneTitle::Harvesting, &mut update_rx);

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
    assert_activity_started(51_000, ui::PaneTitle::Harvesting, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(player.inventory_id(), Food, "Tomato", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn respects_a_produces_minimum_level_requirements() {
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
        _command_tx,
        _command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(1, 2);

    give_player_level(Harvesting, 1, &mut player);
    player.endorse_with(":can_pick");

    let patch_id = get_facility_id_at(1, 1, &map);
    let patch = facilities
        .get_mut(patch_id)
        .expect("unable to locate patch");

    patch.set_property("produce", 7);

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

    assert!(activity.is_none());
}

#[test]
fn respects_a_produces_maximum_level_requirements() {
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
        _command_tx,
        _command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(1, 2);

    give_player_level(Harvesting, 45, &mut player);
    player.endorse_with(":can_pick");

    let patch_id = get_facility_id_at(1, 1, &map);
    let patch = facilities
        .get_mut(patch_id)
        .expect("unable to locate patch");

    patch.set_property("produce", 12);

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

    assert!(activity.is_none());
}

#[test]
fn can_be_exhausted_at_which_point_it_is_removed() {
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
    ) = initialize_game_system_with_player_at(1, 2);

    give_player_level(Harvesting, 40, &mut player);
    player.endorse_with(":can_pick");

    let patch_id = get_facility_id_at(1, 1, &map);
    let patch = facilities
        .get_mut(patch_id)
        .expect("unable to locate patch");

    patch.set_property("produce", 24);
    patch.set_property("quantity", 1);

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
    assert_activity_started(21_000, ui::PaneTitle::Harvesting, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(player.inventory_id(), Food, "Wheat", &mut command_rx);
    assert_is_activity_abort(&mut command_rx);
    assert_is_destroy_facility(&mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn picking_eggplant_provides_20_xp_gain() {
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
    ) = initialize_game_system_with_player_at(1, 2);

    player.endorse_with(":can_pick");
    give_player_level(Harvesting, 15, &mut player);

    let exp_xp = player.get_xp(Harvesting) + 20;

    let patch_id = get_facility_id_at(1, 1, &map);
    let patch = facilities
        .get_mut(patch_id)
        .expect("unable to locate patch");

    patch.set_property("produce", 9);

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

    assert_eq!(player.get_xp(Harvesting), exp_xp);
}
