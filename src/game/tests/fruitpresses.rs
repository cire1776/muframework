use super::*;
use common::timer::TagType;

#[test]
fn fruitpress_can_be_opened() {
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
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(10, 11);

    // set facing to avoid change facing update
    player.facing = Direction::Right;

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
        &Command::Move(Direction::Right, MoveCommandMode::SneakUse),
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    assert!(activity.is_none());

    let facility_id = get_facility_id_at(11, 11, &map);
    assert_external_inventory_opened(&mut vec![], facility_id, &mut update_rx);

    assert_commands_are_empty(&mut command_rx)
}

#[test]
fn apples_can_be_added_to_fruitpress() {
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
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(10, 11);

    clear_inventory(&mut player, &mut inventories, &mut items);

    let item =
        give_player_spawned_items(Food, "Apple", 20, &mut player, &mut inventories, &mut items);

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
        &Command::Move(Direction::Right, MoveCommandMode::SneakUse),
        None,
        None,
    );

    let facility_id = get_facility_id_at(11, 11, &map);
    let player_id = player.id;

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
        activity,
        &Command::TransferItem(item.id, player_id, facility_id),
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    assert!(activity.is_none());

    assert_external_inventory_updated(&mut vec![item], &mut update_rx);
    assert_inventory_updated(&mut vec![], &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn olives_can_be_added_to_fruitpress() {
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
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(10, 11);

    clear_inventory(&mut player, &mut inventories, &mut items);

    let olives =
        give_player_spawned_items(Food, "Olive", 20, &mut player, &mut inventories, &mut items);

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
        &Command::Move(Direction::Right, MoveCommandMode::SneakUse),
        None,
        None,
    );

    let facility_id = get_facility_id_at(11, 11, &map);
    let player_id = player.id;

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
        activity,
        &Command::TransferItem(olives.id, player_id, facility_id),
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    assert!(activity.is_none());

    assert_external_inventory_updated(&mut vec![olives], &mut update_rx);
    assert_inventory_updated(&mut vec![], &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn non_fruit_cannot_be_added_to_fruitpress() {
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
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(10, 11);

    clear_inventory(&mut player, &mut inventories, &mut items);

    let fish = give_player_spawned_items(
        Ingredient,
        "Trout",
        20,
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
        &mut game_data,
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Right, MoveCommandMode::SneakUse),
        None,
        None,
    );

    let facility_id = get_facility_id_at(11, 11, &map);
    let player_id = player.id;

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
        activity,
        &Command::TransferItem(fish.id, player_id, facility_id),
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    assert!(activity.is_none());

    assert_external_inventory_updated(&mut vec![], &mut update_rx);
    assert_inventory_updated(&mut vec![fish], &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn cannot_fill_before_adding_fruit() {
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
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(10, 11);

    player.endorse_with(":can_fill");

    clear_inventory(&mut player, &mut inventories, &mut items);

    give_player_spawned_items(
        Material,
        "Glass Bottle",
        10,
        &mut player,
        &mut inventories,
        &mut items,
    );

    // set facing to avoid change facing update
    player.facing = Direction::Right;

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
        &Command::Move(Direction::Right, MoveCommandMode::Use),
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    assert!(activity.is_none());

    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn can_press_when_apples_have_been_added() {
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
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(10, 11);

    clear_inventory(&mut player, &mut inventories, &mut items);

    let apples =
        give_player_spawned_items(Food, "Apple", 10, &mut player, &mut inventories, &mut items);

    give_player_spawned_items(
        Material,
        "Glass Bottle",
        10,
        &mut player,
        &mut inventories,
        &mut items,
    );

    let facility_id = get_facility_id_at(11, 11, &map);
    let player_id = player.id;

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
        &mut game_data,
        &mut rng,
        &mut timer,
        None,
        &Command::TransferItem(apples.id, player_id, facility_id),
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
        &mut game_data,
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Right, MoveCommandMode::Use),
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    assert!(activity.is_some());

    assert_eq!(
        timer.tags["ActivityComplete"],
        TagType::Duration(chrono::Duration::seconds(60))
    );

    assert_activity_started(60000, Pressing, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);

    let _activity = game_state.game_loop_iteration(
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
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    let facility = facilities.get(facility_id).expect("unable to get facility");
    assert_eq!(facility.get_property("item"), 1);
    assert_eq!(facility.get_property("output"), 1);
}

#[test]
fn can_press_when_olives_have_been_added() {
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
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(10, 11);

    clear_inventory(&mut player, &mut inventories, &mut items);

    let olives =
        give_player_spawned_items(Food, "Olive", 10, &mut player, &mut inventories, &mut items);

    give_player_spawned_items(
        Material,
        "Glass Bottle",
        10,
        &mut player,
        &mut inventories,
        &mut items,
    );

    let facility_id = get_facility_id_at(11, 11, &map);
    let player_id = player.id;

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
        &mut game_data,
        &mut rng,
        &mut timer,
        None,
        &Command::TransferItem(olives.id, player_id, facility_id),
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
        &mut game_data,
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Right, MoveCommandMode::Use),
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    assert!(activity.is_some());

    assert_eq!(
        timer.tags["ActivityComplete"],
        TagType::Duration(chrono::Duration::seconds(60))
    );

    assert_activity_started(60000, Pressing, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);

    let _activity = game_state.game_loop_iteration(
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
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    let facility = facilities.get(facility_id).expect("unable to get facility");
    assert_eq!(facility.get_property("item"), 2);
    assert_eq!(facility.get_property("output"), 1);
}

#[test]
fn can_fill_bottles_when_apple_juice_is_available_aborting_when_empty() {
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
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(10, 11);

    clear_inventory(&mut player, &mut inventories, &mut items);

    player.endorse_with(":can_fill");

    give_player_spawned_items(
        Material,
        "Glass Bottle",
        10,
        &mut player,
        &mut inventories,
        &mut items,
    );

    let facility_id = get_facility_id_at(11, 11, &map);

    let facility = facilities
        .get_mut(facility_id)
        .expect("unable to get fruitpress.");

    facility.set_property("output", 1);
    facility.set_property("item", 1);

    // set facing to avoid change facing update
    player.facing = Direction::Right;

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
        &Command::Move(Direction::Right, MoveCommandMode::Use),
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    assert!(activity.is_some());

    assert_eq!(
        timer.tags["ActivityComplete"],
        TagType::Duration(chrono::Duration::seconds(30))
    );

    assert_activity_started(30000, Filling, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);

    let _activity = game_state.game_loop_iteration(
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
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    assert_activity_expired(&mut update_rx);
    assert_activity_started(30000, Filling, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(player.inventory_id(), Food, "Apple Juice", &mut command_rx);
    assert_is_activity_abort(&mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn can_fill_bottles_when_olive_oil_is_available_aborting_when_empty() {
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
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(10, 11);

    clear_inventory(&mut player, &mut inventories, &mut items);

    player.endorse_with(":can_fill");

    give_player_spawned_items(
        Material,
        "Glass Bottle",
        10,
        &mut player,
        &mut inventories,
        &mut items,
    );

    let facility_id = get_facility_id_at(11, 11, &map);

    let facility = facilities
        .get_mut(facility_id)
        .expect("unable to get fruitpress.");

    facility.set_property("output", 1);
    facility.set_property("item", 2);

    // set facing to avoid change facing update
    player.facing = Direction::Right;

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
        &Command::Move(Direction::Right, MoveCommandMode::Use),
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    assert!(activity.is_some());

    assert_eq!(
        timer.tags["ActivityComplete"],
        TagType::Duration(chrono::Duration::seconds(30))
    );

    assert_activity_started(30000, Filling, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);

    let _activity = game_state.game_loop_iteration(
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
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    assert_activity_expired(&mut update_rx);
    assert_activity_started(30000, Filling, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(player.inventory_id(), Food, "Olive Oil", &mut command_rx);
    assert_is_activity_abort(&mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}
