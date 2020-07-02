use super::*;

#[test]
fn can_dig_without_success() {
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
    ) = initialize_game_system_with_player_at(13, 12);

    rng.set_fail("water_chance");
    rng.set_fail("oil_chance");
    rng.set_fail("bedrock_chance");

    player.endorse_with(":can_dig");
    give_player_level(Engineering, 1, &mut player);

    let facility = facilities
        .get(get_facility_id_at(13, 11, &&map))
        .unwrap()
        .clone();

    let exp_depth = facility.get_property("depth") + 1;

    let mut activity = game_state.game_loop_iteration(
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
        Some(command_tx.clone()),
    );

    assert_eq!(
        timer.tags["ActivityComplete"],
        chrono::Duration::seconds(60)
    );
    assert!(activity.is_some());

    assert_activity_started(60000, Digging, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);

    activity = game_state.game_loop_iteration(
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

    assert!(activity.is_some());

    assert_activity_expired(&mut update_rx);
    assert_activity_started(60000, Digging, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);

    let facility = facilities
        .get(get_facility_id_at(13, 11, &&map))
        .unwrap()
        .clone();

    assert_eq!(facility.get_property("depth"), exp_depth);
    assert_eq!(facility.get_property("fluid"), 0);
}

#[test]
fn can_dig_striking_water() {
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
    ) = initialize_game_system_with_player_at(13, 12);

    rng.set_succeed("water_chance");
    rng.set_fail("oil_chance");

    player.endorse_with(":can_dig");
    give_player_level(Engineering, 1, &mut player);

    let facility = facilities
        .get(get_facility_id_at(13, 11, &map))
        .unwrap()
        .clone();
    let exp_depth = facility.get_property("depth") + 1;

    let mut activity = game_state.game_loop_iteration(
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

    assert_eq!(
        timer.tags["ActivityComplete"],
        chrono::Duration::seconds(60)
    );
    assert!(activity.is_some());

    assert_activity_started(60000, Digging, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);

    activity = game_state.game_loop_iteration(
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

    assert!(activity.is_some());

    assert_activity_expired(&mut update_rx);
    assert_activity_started(60000, Digging, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_activity_abort(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);

    let facility = facilities
        .get(get_facility_id_at(13, 11, &&map))
        .unwrap()
        .clone();

    assert_eq!(facility.get_property("depth"), exp_depth);
    assert_eq!(facility.get_property("fluid"), 1);
}

#[test]
fn can_dig_striking_oil() {
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
    ) = initialize_game_system_with_player_at(13, 12);

    rng.set_fail("water_chance");
    rng.set_succeed("oil_chance");
    rng.set_fail("bedrock_chance");

    player.endorse_with(":can_dig");
    give_player_level(Engineering, 1, &mut player);

    let facility = facilities
        .get(get_facility_id_at(13, 11, &map))
        .unwrap()
        .clone();
    let exp_depth = facility.get_property("depth") + 1;

    let mut activity = game_state.game_loop_iteration(
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

    assert_eq!(
        timer.tags["ActivityComplete"],
        chrono::Duration::seconds(60)
    );
    assert!(activity.is_some());

    assert_activity_started(60000, Digging, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);

    activity = game_state.game_loop_iteration(
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

    assert!(activity.is_some());

    assert_activity_expired(&mut update_rx);
    assert_activity_started(60000, Digging, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_activity_abort(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);

    let facility = facilities
        .get(get_facility_id_at(13, 11, &&map))
        .unwrap()
        .clone();

    assert_eq!(facility.get_property("depth"), exp_depth);
    assert_eq!(facility.get_property("fluid"), 2);
}

#[test]
fn can_dig_striking_bedrock() {
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
    ) = initialize_game_system_with_player_at(13, 12);

    rng.set_fail("water_chance");
    rng.set_fail("oil_chance");
    rng.set_succeed("bedrock_chance");

    player.endorse_with(":can_dig");
    give_player_level(Engineering, 1, &mut player);

    let facility = facilities
        .get(get_facility_id_at(13, 11, &map))
        .unwrap()
        .clone();
    let exp_depth = facility.get_property("depth") + 1;

    let mut activity = game_state.game_loop_iteration(
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

    assert_eq!(
        timer.tags["ActivityComplete"],
        chrono::Duration::seconds(60)
    );
    assert!(activity.is_some());

    assert_activity_started(60000, Digging, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);

    activity = game_state.game_loop_iteration(
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

    assert!(activity.is_some());

    assert_activity_expired(&mut update_rx);
    assert_activity_started(60000, Digging, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_activity_abort(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);

    let facility = facilities
        .get(get_facility_id_at(13, 11, &&map))
        .unwrap()
        .clone();

    assert_eq!(facility.get_property("depth"), exp_depth);
    assert_eq!(facility.get_property("fluid"), 255);
}

#[test]
fn can_fill_from_water_well() {
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
    ) = initialize_game_system_with_player_at(14, 12);

    player.endorse_with(":can_fill");

    equip_player_with_spawned_item(
        Material,
        "Glass Bottle",
        &mut player,
        &mut inventories,
        &mut items,
    );

    let mut activity = game_state.game_loop_iteration(
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

    assert_eq!(
        timer.tags["ActivityComplete"],
        chrono::Duration::seconds(30)
    );
    assert!(activity.is_some());

    assert_activity_started(30000, Filling, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);

    activity = game_state.game_loop_iteration(
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

    assert!(activity.is_some());

    assert_activity_expired(&mut update_rx);
    assert_activity_started(30000, Filling, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(player.id, Ingredient, "Bottle of Water", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn can_fill_from_oil_well() {
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
    ) = initialize_game_system_with_player_at(15, 12);

    player.endorse_with(":can_fill");

    equip_player_with_spawned_item(
        Material,
        "Glass Bottle",
        &mut player,
        &mut inventories,
        &mut items,
    );

    let mut activity = game_state.game_loop_iteration(
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

    assert_eq!(
        timer.tags["ActivityComplete"],
        chrono::Duration::seconds(30)
    );
    assert!(activity.is_some());

    assert_activity_started(30000, Filling, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);

    activity = game_state.game_loop_iteration(
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

    assert!(activity.is_some());

    assert_activity_expired(&mut update_rx);
    assert_activity_started(30000, Filling, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(player.id, Material, "Bottle of Oil", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn cannot_fill_from_dry_well() {
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
    ) = initialize_game_system_with_player_at(13, 12);

    player.endorse_with(":can_fill");

    equip_player_with_spawned_item(
        Material,
        "Glass Bottle",
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

    assert_eq!(timer.tags.get("ActivityComplete"), None);
    assert!(activity.is_none());

    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn level_1_cannot_mine_a_level_2_well() {
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
    ) = initialize_game_system_with_player_at(13, 12);

    player.endorse_with(":can_dig");
    give_player_level(Engineering, 1, &mut player);

    let facility_id = get_facility_id_at(13, 11, &map);

    let facility = facilities
        .get_mut(facility_id)
        .expect("unable to find facility");

    facility.set_property("depth", 300);

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

    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}
#[test]
fn stops_after_the_well_grows_too_deep_for_level_1() {
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
    ) = initialize_game_system_with_player_at(13, 12);

    rng.set_fail("water_chance");
    rng.set_fail("oil_chance");
    rng.set_fail("bedrock_chance");

    player.endorse_with(":can_dig");
    give_player_level(Engineering, 1, &mut player);

    let facility_id = get_facility_id_at(13, 11, &map);

    let facility = facilities
        .get_mut(facility_id)
        .expect("unable to find facility");

    facility.set_property("depth", 299);

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
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    // assert!(activity.is_none());

    assert_activity_expired(&mut update_rx);
    assert_activity_started(60000, Digging, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_activity_abort(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn stops_after_the_well_grows_too_deep_for_level_29() {
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
    ) = initialize_game_system_with_player_at(13, 12);

    rng.set_fail("water_chance");
    rng.set_fail("oil_chance");
    rng.set_fail("bedrock_chance");

    player.endorse_with(":can_dig");
    give_player_level(Engineering, 29, &mut player);

    let facility_id = get_facility_id_at(13, 11, &map);

    let facility = facilities
        .get_mut(facility_id)
        .expect("unable to find facility");

    facility.set_property("depth", 649);

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
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());

    assert_activity_expired(&mut update_rx);
    assert_activity_started(32_000, Digging, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_activity_abort(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn stops_after_the_well_grows_too_deep_for_level_45() {
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
    ) = initialize_game_system_with_player_at(13, 12);

    rng.set_fail("water_chance");
    rng.set_fail("oil_chance");
    rng.set_fail("bedrock_chance");

    player.endorse_with(":can_dig");
    give_player_level(Engineering, 45, &mut player);

    let facility_id = get_facility_id_at(13, 11, &map);

    let facility = facilities
        .get_mut(facility_id)
        .expect("unable to find facility");

    facility.set_property("depth", 999);

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
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());

    assert_activity_expired(&mut update_rx);
    assert_activity_started(16_000, Digging, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_activity_abort(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn gives_10_xp_for_digging_level_1_well() {
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
    ) = initialize_game_system_with_player_at(13, 12);

    rng.set_fail("water_chance");
    rng.set_fail("oil_chance");
    rng.set_fail("bedrock_chance");

    let exp_xp = player.get_xp(Engineering) + 10;

    player.endorse_with(":can_dig");
    give_player_level(Engineering, 45, &mut player);

    let facility_id = get_facility_id_at(13, 11, &map);

    let facility = facilities
        .get_mut(facility_id)
        .expect("unable to find facility");

    facility.set_property("depth", 0);

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
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    let actual_xp = player.get_xp(Engineering);

    assert_eq!(actual_xp, exp_xp);
}

#[test]
fn gives_18_xp_for_digging_level_2_well() {
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
    ) = initialize_game_system_with_player_at(13, 12);

    rng.set_fail("water_chance");
    rng.set_fail("oil_chance");
    rng.set_fail("bedrock_chance");

    let exp_xp = player.get_xp(Engineering) + 18;

    player.endorse_with(":can_dig");
    give_player_level(Engineering, 45, &mut player);

    let facility_id = get_facility_id_at(13, 11, &map);

    let facility = facilities
        .get_mut(facility_id)
        .expect("unable to find facility");

    facility.set_property("depth", 300);

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
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    let actual_xp = player.get_xp(Engineering);

    assert_eq!(actual_xp, exp_xp);
}

#[test]
fn gives_23_xp_for_digging_level_3_well() {
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
    ) = initialize_game_system_with_player_at(13, 12);

    rng.set_fail("water_chance");
    rng.set_fail("oil_chance");
    rng.set_fail("bedrock_chance");

    let exp_xp = player.get_xp(Engineering) + 23;

    player.endorse_with(":can_dig");
    give_player_level(Engineering, 45, &mut player);

    let facility_id = get_facility_id_at(13, 11, &map);

    let facility = facilities
        .get_mut(facility_id)
        .expect("unable to find facility");

    facility.set_property("depth", 900);

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
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    let actual_xp = player.get_xp(Engineering);

    assert_eq!(actual_xp, exp_xp);
}

#[test]
fn gives_100_xp_bonus_for_completing_level_1_well() {
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
    ) = initialize_game_system_with_player_at(13, 12);

    rng.set_succeed("water_chance");
    rng.set_fail("oil_chance");
    rng.set_fail("bedrock_chance");

    let exp_xp = player.get_xp(Engineering) + 110;

    player.endorse_with(":can_dig");
    give_player_level(Engineering, 45, &mut player);

    let facility_id = get_facility_id_at(13, 11, &map);

    let facility = facilities
        .get_mut(facility_id)
        .expect("unable to find facility");

    facility.set_property("depth", 0);

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
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    let actual_xp = player.get_xp(Engineering);

    assert_eq!(actual_xp, exp_xp);
}

#[test]
fn gives_200_xp_bonus_for_completing_level_2_well() {
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
    ) = initialize_game_system_with_player_at(13, 12);

    rng.set_succeed("water_chance");
    rng.set_fail("oil_chance");
    rng.set_fail("bedrock_chance");

    let exp_xp = player.get_xp(Engineering) + 218;

    player.endorse_with(":can_dig");
    give_player_level(Engineering, 45, &mut player);

    let facility_id = get_facility_id_at(13, 11, &map);

    let facility = facilities
        .get_mut(facility_id)
        .expect("unable to find facility");

    facility.set_property("depth", 300);

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
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    let actual_xp = player.get_xp(Engineering);

    assert_eq!(actual_xp, exp_xp);
}

#[test]
fn gives_300_xp_bonus_for_completing_level_3_well() {
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
    ) = initialize_game_system_with_player_at(13, 12);

    rng.set_succeed("water_chance");
    rng.set_fail("oil_chance");
    rng.set_fail("bedrock_chance");

    let exp_xp = player.get_xp(Engineering) + 323;

    player.endorse_with(":can_dig");
    give_player_level(Engineering, 45, &mut player);

    let facility_id = get_facility_id_at(13, 11, &map);

    let facility = facilities
        .get_mut(facility_id)
        .expect("unable to find facility");

    facility.set_property("depth", 650);

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
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    let actual_xp = player.get_xp(Engineering);

    assert_eq!(actual_xp, exp_xp);
}

#[test]
fn gives_1000_xp_bonus_for_completing_level_1_oil_well() {
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
    ) = initialize_game_system_with_player_at(13, 12);

    rng.set_fail("water_chance");
    rng.set_succeed("oil_chance");
    rng.set_fail("bedrock_chance");

    let exp_xp = player.get_xp(Engineering) + 1010;

    player.endorse_with(":can_dig");
    give_player_level(Engineering, 45, &mut player);

    let facility_id = get_facility_id_at(13, 11, &map);

    let facility = facilities
        .get_mut(facility_id)
        .expect("unable to find facility");

    facility.set_property("depth", 0);

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
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    let actual_xp = player.get_xp(Engineering);

    assert_eq!(actual_xp, exp_xp);
}

#[test]
fn gives_2000_xp_bonus_for_completing_level_2_oil_well() {
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
    ) = initialize_game_system_with_player_at(13, 12);

    rng.set_fail("water_chance");
    rng.set_succeed("oil_chance");
    rng.set_fail("bedrock_chance");

    let exp_xp = player.get_xp(Engineering) + 2018;

    player.endorse_with(":can_dig");
    give_player_level(Engineering, 45, &mut player);

    let facility_id = get_facility_id_at(13, 11, &map);

    let facility = facilities
        .get_mut(facility_id)
        .expect("unable to find facility");

    facility.set_property("depth", 500);

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
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    let actual_xp = player.get_xp(Engineering);

    assert_eq!(actual_xp, exp_xp);
}
#[test]
fn gives_3000_xp_bonus_for_completing_level_3_oil_well() {
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
    ) = initialize_game_system_with_player_at(13, 12);

    rng.set_fail("water_chance");
    rng.set_succeed("oil_chance");
    rng.set_fail("bedrock_chance");

    let exp_xp = player.get_xp(Engineering) + 3023;

    player.endorse_with(":can_dig");
    give_player_level(Engineering, 45, &mut player);

    let facility_id = get_facility_id_at(13, 11, &map);

    let facility = facilities
        .get_mut(facility_id)
        .expect("unable to find facility");

    facility.set_property("depth", 900);

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
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    let actual_xp = player.get_xp(Engineering);

    assert_eq!(actual_xp, exp_xp);
}

#[test]
fn gives_15000_xp_bonus_for_completing_level_1_bedrock() {
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
    ) = initialize_game_system_with_player_at(13, 12);

    rng.set_fail("water_chance");
    rng.set_fail("oil_chance");
    rng.set_succeed("bedrock_chance");

    let exp_xp = player.get_xp(Engineering) + 15010;

    player.endorse_with(":can_dig");
    give_player_level(Engineering, 45, &mut player);

    let facility_id = get_facility_id_at(13, 11, &map);

    let facility = facilities
        .get_mut(facility_id)
        .expect("unable to find facility");

    facility.set_property("depth", 100);

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
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    let actual_xp = player.get_xp(Engineering);

    assert_eq!(actual_xp, exp_xp);
}

#[test]
fn gives_10000_xp_bonus_for_completing_level_2_bedrock() {
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
    ) = initialize_game_system_with_player_at(13, 12);

    rng.set_fail("water_chance");
    rng.set_fail("oil_chance");
    rng.set_succeed("bedrock_chance");

    let exp_xp = player.get_xp(Engineering) + 10018;

    player.endorse_with(":can_dig");
    give_player_level(Engineering, 45, &mut player);

    let facility_id = get_facility_id_at(13, 11, &map);

    let facility = facilities
        .get_mut(facility_id)
        .expect("unable to find facility");

    facility.set_property("depth", 400);

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
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    let actual_xp = player.get_xp(Engineering);

    assert_eq!(actual_xp, exp_xp);
}

#[test]
fn gives_5000_xp_bonus_for_completing_level_3_bedrock() {
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
    ) = initialize_game_system_with_player_at(13, 12);

    rng.set_fail("water_chance");
    rng.set_fail("oil_chance");
    rng.set_succeed("bedrock_chance");

    let exp_xp = player.get_xp(Engineering) + 5023;

    player.endorse_with(":can_dig");
    give_player_level(Engineering, 45, &mut player);

    let facility_id = get_facility_id_at(13, 11, &map);

    let facility = facilities
        .get_mut(facility_id)
        .expect("unable to find facility");

    facility.set_property("depth", 800);

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
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    let actual_xp = player.get_xp(Engineering);

    assert_eq!(actual_xp, exp_xp);
}
