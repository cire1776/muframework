use super::*;

use common::timer::TagType;
use ui::PaneTitle;

#[test]
fn without_endorsement_it_wont_allow_you_to_fish() {
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
    ) = initialize_game_system_with_player_at(14, 1);

    player.facing = Direction::Right;

    give_player_level(Fishing, 1, &mut player);

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
        None,
    );

    assert_player_is_at(14, 1, &player);

    assert!(activity.is_none());

    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn it_allows_rod_fishing_with_the_can_fish_endorsement() {
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
    ) = initialize_game_system_with_player_at(14, 1);

    player.facing = Direction::Right;
    player.endorse_with(":can_fish");
    give_player_level(Fishing, 1, &mut player);

    equip_player_with_spawned_item(
        Tool,
        "Simple Fishing Rod",
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
        &Command::Move(Direction::Right, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_player_is_at(14, 1, &player);

    assert_eq!(timer.tags["ActivityComplete"], TagType::Ticks(60 * 60));
    assert!(activity.is_some());

    assert_activity_started(60_000, PaneTitle::Fishing, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);

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
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());
    assert_activity_expired(&mut update_rx);
    assert_xp_is_updated(player.id, Fishing, 4, &mut update_rx);
    assert_activity_started(60_000, PaneTitle::Fishing, &mut update_rx);
    assert_is_spawning_item(player.id, Ingredient, "Mackeral", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn it_allows_net_fishing_for_shrimp_with_the_can_net_fish_endorsement() {
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
    ) = initialize_game_system_with_player_at(14, 1);
    rng.set_succeed("fish_type");
    player.facing = Direction::Right;
    player.endorse_with(":can_net_fish");
    give_player_level(Fishing, 1, &mut player);

    equip_player_with_spawned_item(
        Tool,
        "Simple Fishing Net",
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
        &Command::Move(Direction::Right, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_player_is_at(14, 1, &player);

    assert_eq!(timer.tags["ActivityComplete"], TagType::Ticks(45 * 60));
    assert!(activity.is_some());

    assert_activity_started(45_000, PaneTitle::NetFishing, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);

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
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());
    assert_activity_expired(&mut update_rx);
    assert_xp_is_updated(player.id, Fishing, 3, &mut update_rx);
    assert_activity_started(45_000, PaneTitle::NetFishing, &mut update_rx);
    assert_is_spawning_item(player.id, Ingredient, "Shrimp", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn it_allows_net_fishing_for_frogs_with_the_can_net_fish_endorsement() {
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
    ) = initialize_game_system_with_player_at(14, 1);
    rng.set_fail("fish_type");
    player.facing = Direction::Right;
    player.endorse_with(":can_net_fish");
    give_player_level(Fishing, 1, &mut player);

    equip_player_with_spawned_item(
        Tool,
        "Simple Fishing Net",
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
        &Command::Move(Direction::Right, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_player_is_at(14, 1, &player);

    assert_eq!(timer.tags["ActivityComplete"], TagType::Ticks(45 * 60));
    assert!(activity.is_some());

    assert_activity_started(45_000, PaneTitle::NetFishing, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);

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
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());
    assert_activity_expired(&mut update_rx);
    assert_xp_is_updated(player.id, Fishing, 3, &mut update_rx);
    assert_activity_started(45_000, PaneTitle::NetFishing, &mut update_rx);
    assert_is_spawning_item(player.id, Ingredient, "Frog", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn it_allows_placing_a_trap_with_endorsement_can_place_fishing_trap() {
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
    ) = initialize_game_system_with_player_at(14, 1);
    rng.set_fail("fish_type");
    player.facing = Direction::Right;
    player.endorse_with(":can_place_fishing_trap");
    give_player_level(Fishing, 1, &mut player);

    equip_player_with_spawned_item(
        Tool,
        "Simple Fishing Trap",
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
        &Command::Move(Direction::Right, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_player_is_at(14, 1, &player);

    assert_eq!(timer.tags["ActivityComplete"], TagType::Ticks(30 * 60));
    assert!(activity.is_some());

    assert_activity_started(30_000, PaneTitle::PlacingTrap, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);

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
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    let fishing_spot_id = get_facility_id_at(15, 1, &map);

    assert!(activity.is_some());
    assert_activity_expired(&mut update_rx);
    assert_activity_started(30000, ui::PaneTitle::PlacingTrap, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_transfer_equipement_to_inventory(
        MountingPoint::OnHand,
        fishing_spot_id,
        &mut command_rx,
    );
    assert_is_activity_abort(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn it_transfers_fishing_trap_from_equipment_to_fishing_spot() {
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
    ) = initialize_game_system_with_player_at(14, 1);
    rng.set_fail("fish_type");
    player.facing = Direction::Right;
    player.endorse_with(":can_place_fishing_trap");
    give_player_level(Fishing, 1, &mut player);

    equip_player_with_spawned_item(
        Tool,
        "Simple Fishing Trap",
        &mut player,
        &mut inventories,
        &mut items,
    );

    let fishing_spot_id = get_facility_id_at(15, 1, &map);

    game_state.game_loop_iteration(
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
        &Command::TransferEquipmentToInventory(MountingPoint::OnHand, fishing_spot_id),
        Some(&update_tx),
        None,
    );

    assert_is_equipment_updated(vec![], &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn it_does_not_allow_use_of_fishing_spot_during_waiting_period() {
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
        _update_rx,
        command_tx,
        _command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(14, 1);
    rng.set_fail("fish_type");
    player.facing = Direction::Right;
    player.endorse_with(":can_place_fishing_trap");
    give_player_level(Fishing, 1, &mut player);

    equip_player_with_spawned_item(
        Tool,
        "Simple Fishing Trap",
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
        &Command::Move(Direction::Right, MoveCommandMode::Use),
        Some(&update_tx),
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
        activity,
        &Command::ActivityComplete,
        None,
        None,
    );

    player.endorse_with(":can_fish");

    equip_player_with_spawned_item(
        Tool,
        "Simple Fishing Rod",
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
        activity,
        &Command::Move(Direction::Right, MoveCommandMode::Use),
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_none());
}

#[test]
fn it_allows_retreiving_a_trap_after_the_expiration() {
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
    ) = initialize_game_system_with_player_at(14, 1);
    rng.set_fail("fish_type");
    player.facing = Direction::Right;
    player.endorse_with(":can_place_fishing_trap");
    give_player_level(Fishing, 1, &mut player);

    equip_player_with_spawned_item(
        Tool,
        "Simple Fishing Trap",
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
        &Command::Move(Direction::Right, MoveCommandMode::Use),
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
        &mut game_data,
        &mut rng,
        &mut timer,
        activity,
        &Command::ActivityComplete,
        None,
        None,
    );

    let fishing_spot = get_facility_at(15, 1, &map, &mut facilities);
    fishing_spot.set_property("trap_expiration", 1);
    player.unendorse_with(":can_place_fishing_trap");

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
        None,
    );

    assert!(activity.is_some());

    assert_eq!(timer.tags["ActivityComplete"], TagType::Ticks(30 * 60));
    assert!(activity.is_some());

    assert_activity_started(30_000, PaneTitle::CollectingTrap, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn fishing_spot_produce_single_crab_with_trap() {
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
    ) = initialize_game_system_with_player_at(14, 1);
    rng.set_fail("fish_type");
    player.facing = Direction::Right;
    player.endorse_with(":can_place_fishing_trap");
    give_player_level(Fishing, 1, &mut player);

    let trap_id = equip_player_with_spawned_item(
        Tool,
        "Simple Fishing Trap",
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
        &Command::Move(Direction::Right, MoveCommandMode::Use),
        None,
        None,
    );

    assert!(activity.is_some());

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
        &Command::ActivityComplete,
        None,
        None,
    );
    let fishing_spot_id = {
        let fishing_spot = get_facility_at(15, 1, &map, &mut facilities);
        fishing_spot.set_property("trap_expiration", 1);
        fishing_spot.id
    };

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
        &Command::TransferEquipmentToInventory(MountingPoint::OnHand, fishing_spot_id),
        None,
        None,
    );

    player.unendorse_with(":can_place_fishing_trap");

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
        None,
        None,
    );

    assert!(activity.is_some());

    rng.set("trap_spawn", 1);

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
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());

    assert_activity_expired(&mut update_rx);
    assert_activity_started(30000, ui::PaneTitle::CollectingTrap, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_activity_abort(&mut command_rx);
    assert_is_spawning_item(player.inventory_id(), Ingredient, "Crab", &mut command_rx);
    assert_transfer_item(
        trap_id,
        fishing_spot_id,
        player.inventory_id(),
        &mut command_rx,
    );
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn trap_can_spawn_upto_five_crab() {
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
    ) = initialize_game_system_with_player_at(14, 1);
    rng.set_fail("fish_type");
    player.facing = Direction::Right;
    player.endorse_with(":can_place_fishing_trap");
    give_player_level(Fishing, 1, &mut player);

    let trap_id = equip_player_with_spawned_item(
        Tool,
        "Simple Fishing Trap",
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
        &Command::Move(Direction::Right, MoveCommandMode::Use),
        None,
        None,
    );

    assert!(activity.is_some());

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
        &Command::ActivityComplete,
        None,
        None,
    );
    let fishing_spot_id = {
        let fishing_spot = get_facility_at(15, 1, &map, &mut facilities);
        fishing_spot.set_property("trap_expiration", 1);
        fishing_spot.id
    };

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
        &Command::TransferEquipmentToInventory(MountingPoint::OnHand, fishing_spot_id),
        None,
        None,
    );

    player.unendorse_with(":can_place_fishing_trap");

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
        None,
        None,
    );

    assert!(activity.is_some());

    rng.set("trap_spawn", 5);

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
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());

    assert_activity_expired(&mut update_rx);
    assert_activity_started(30000, ui::PaneTitle::CollectingTrap, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_activity_abort(&mut command_rx);
    assert_is_spawning_item(player.inventory_id(), Ingredient, "Crab", &mut command_rx);
    assert_is_spawning_item(player.inventory_id(), Ingredient, "Crab", &mut command_rx);
    assert_is_spawning_item(player.inventory_id(), Ingredient, "Crab", &mut command_rx);
    assert_is_spawning_item(player.inventory_id(), Ingredient, "Crab", &mut command_rx);
    assert_is_spawning_item(player.inventory_id(), Ingredient, "Crab", &mut command_rx);
    assert_transfer_item(
        trap_id,
        fishing_spot_id,
        player.inventory_id(),
        &mut command_rx,
    );
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn net_fishing_above_level_produces_seaweed() {
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
    ) = initialize_game_system_with_player_at(15, 2);
    rng.set_fail("fish_type");
    rng.set_succeed("flotsam_type");

    player.facing = Direction::Right;

    player.endorse_with(":can_net_fish");
    give_player_level(Fishing, 1, &mut player);

    equip_player_with_spawned_item(
        Tool,
        "Simple Fishing Net",
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
        &Command::Move(Direction::Right, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_eq!(timer.tags["ActivityComplete"], TagType::Ticks(45 * 60));
    assert!(activity.is_some());

    assert_activity_started(45_000, PaneTitle::NetFishing, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);

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
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());
    assert_activity_expired(&mut update_rx);
    assert_xp_is_updated(player.id, Fishing, 1, &mut update_rx);
    assert_activity_started(45_000, PaneTitle::NetFishing, &mut update_rx);
    assert_is_spawning_item(player.id, Ingredient, "Seaweed", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn net_fishing_above_level_produces_driftwood() {
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
    ) = initialize_game_system_with_player_at(15, 2);
    rng.set_fail("fish_type");
    rng.set_fail("flotsam_type");

    player.facing = Direction::Right;

    player.endorse_with(":can_net_fish");
    give_player_level(Fishing, 1, &mut player);

    equip_player_with_spawned_item(
        Tool,
        "Simple Fishing Net",
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
        &Command::Move(Direction::Right, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_eq!(timer.tags["ActivityComplete"], TagType::Ticks(45 * 60));
    assert!(activity.is_some());

    assert_activity_started(45_000, PaneTitle::NetFishing, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);

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
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());
    assert_activity_expired(&mut update_rx);
    assert_xp_is_updated(player.id, Fishing, 1, &mut update_rx);
    assert_activity_started(45_000, PaneTitle::NetFishing, &mut update_rx);
    assert_is_spawning_item(player.id, Material, "Driftwood", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn rod_fishing_above_level_produces_seaweed() {
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
    ) = initialize_game_system_with_player_at(15, 2);
    rng.set_fail("fish_type");
    rng.set_succeed("flotsam_type");

    player.facing = Direction::Right;

    player.endorse_with(":can_fish");
    give_player_level(Fishing, 1, &mut player);

    equip_player_with_spawned_item(
        Tool,
        "Simple Fishing Rod",
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
        &Command::Move(Direction::Right, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_eq!(timer.tags["ActivityComplete"], TagType::Ticks(60 * 60));
    assert!(activity.is_some());

    assert_activity_started(60_000, PaneTitle::Fishing, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);

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
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());
    assert_activity_expired(&mut update_rx);
    assert_xp_is_updated(player.id, Fishing, 1, &mut update_rx);
    assert_activity_started(60_000, PaneTitle::Fishing, &mut update_rx);
    assert_is_spawning_item(player.id, Ingredient, "Seaweed", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn rod_fishing_above_level_produces_driftwood() {
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
    ) = initialize_game_system_with_player_at(15, 2);
    rng.set_fail("fish_type");
    rng.set_fail("flotsam_type");

    player.facing = Direction::Right;

    player.endorse_with(":can_fish");
    give_player_level(Fishing, 1, &mut player);

    equip_player_with_spawned_item(
        Tool,
        "Simple Fishing Rod",
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
        &Command::Move(Direction::Right, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_eq!(timer.tags["ActivityComplete"], TagType::Ticks(60 * 60));
    assert!(activity.is_some());

    assert_activity_started(60_000, PaneTitle::Fishing, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);

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
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());
    assert_activity_expired(&mut update_rx);
    assert_xp_is_updated(player.id, Fishing, 1, &mut update_rx);
    assert_activity_started(60_000, PaneTitle::Fishing, &mut update_rx);
    assert_is_spawning_item(player.id, Material, "Driftwood", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn rod_fishing_fails_give_1_xp() {
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
        _update_rx,
        command_tx,
        _command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(15, 2);
    rng.set_fail("fish_type");
    rng.set_fail("flotsam_type");

    player.facing = Direction::Right;

    let exp_xp = player.get_xp(Fishing) + 1;

    player.endorse_with(":can_fish");
    give_player_level(Fishing, 1, &mut player);

    equip_player_with_spawned_item(
        Tool,
        "Simple Fishing Rod",
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
        &Command::Move(Direction::Right, MoveCommandMode::Use),
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
        &mut game_data,
        &mut rng,
        &mut timer,
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert_eq!(player.get_xp(Fishing), exp_xp);
}

#[test]
fn net_fishing_fails_give_1_xp() {
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
        _update_rx,
        command_tx,
        _command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(15, 2);
    rng.set_fail("fish_type");
    rng.set_fail("flotsam_type");

    player.facing = Direction::Right;

    let exp_xp = player.get_xp(Fishing) + 1;

    player.endorse_with(":can_net_fish");
    give_player_level(Fishing, 1, &mut player);

    equip_player_with_spawned_item(
        Tool,
        "Simple Fishing Rod",
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
        &Command::Move(Direction::Right, MoveCommandMode::Use),
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
        &mut game_data,
        &mut rng,
        &mut timer,
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert_eq!(player.get_xp(Fishing), exp_xp);
}

#[test]
fn rod_fishing_mackeral_give_4_xp() {
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
        _update_rx,
        command_tx,
        _command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(14, 1);
    rng.set_fail("fish_type");
    rng.set_fail("flotsam_type");

    player.facing = Direction::Right;

    let exp_xp = player.get_xp(Fishing) + 4;

    player.endorse_with(":can_fish");
    give_player_level(Fishing, 1, &mut player);

    equip_player_with_spawned_item(
        Tool,
        "Simple Fishing Rod",
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
        &Command::Move(Direction::Right, MoveCommandMode::Use),
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
        &mut game_data,
        &mut rng,
        &mut timer,
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert_eq!(player.get_xp(Fishing), exp_xp);
}

#[test]
fn rod_fishing_salmon_give_5_xp() {
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
        _update_rx,
        command_tx,
        _command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(15, 2);
    rng.set_succeed("fish_type");
    rng.set_fail("flotsam_type");

    player.facing = Direction::Right;

    let exp_xp = player.get_xp(Fishing) + 5;

    player.endorse_with(":can_fish");
    give_player_level(Fishing, 5, &mut player);

    equip_player_with_spawned_item(
        Tool,
        "Simple Fishing Rod",
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
        &Command::Move(Direction::Right, MoveCommandMode::Use),
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
        &mut game_data,
        &mut rng,
        &mut timer,
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert_eq!(player.get_xp(Fishing), exp_xp);
}
