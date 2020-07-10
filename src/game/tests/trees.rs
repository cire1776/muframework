use super::*;
use common::timer::TagType;

#[test]
fn can_chop_pine_tree() {
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
    ) = initialize_game_system_with_player_at(10, 8);

    player.endorse_with(":can_chop");

    let exp_xp = player.get_xp(Logging) + 5;

    // set facing to avoid change facing update
    player.facing = Direction::Left;

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
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_eq!(
        timer.tags["ActivityComplete"],
        TagType::Duration(chrono::Duration::seconds(60))
    );

    assert_activity_started(60_000, ui::pane::PaneTitle::Logging, &mut update_rx);
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
    assert_xp_is_updated(player.id, Logging, 5, &mut update_rx);
    assert_activity_started(60_000, ui::pane::PaneTitle::Logging, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(1, Material, "Softwood Log", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);

    assert_eq!(player.get_xp(Logging), exp_xp);
}

#[test]
fn can_chop_apple_tree() {
    let (
        mut player,
        mut map,
        mut obstacles,
        mut characters,
        mut item_class_specifiers,
        mut items,
        mut facilities,
        mut inventories,
        mut _rng,
        mut timer,
        update_tx,
        mut update_rx,
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(10, 9);

    let mut rng = Rng::new();

    player.endorse_with(":can_chop");

    let exp_xp = player.get_xp(Logging) + 6;

    // set facing to avoid change facing update
    player.facing = Direction::Left;

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
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_eq!(
        timer.tags["ActivityComplete"],
        TagType::Duration(chrono::Duration::seconds(60))
    );

    assert_activity_started(60_000, ui::pane::PaneTitle::Logging, &mut update_rx);

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
    assert_xp_is_updated(player.id, Logging, 6, &mut update_rx);
    assert_activity_started(60_000, ui::pane::PaneTitle::Logging, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(1, Material, "Hardwood Log", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);

    assert_eq!(player.get_xp(Logging), exp_xp);
}

#[test]
fn can_chop_olive_tree() {
    let (
        mut player,
        mut map,
        mut obstacles,
        mut characters,
        mut item_class_specifiers,
        mut items,
        mut facilities,
        mut inventories,
        mut _rng,
        mut timer,
        update_tx,
        mut update_rx,
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(10, 10);

    let mut rng = Rng::new();

    player.endorse_with(":can_chop");

    let exp_xp = player.get_xp(Logging) + 6;

    // set facing to avoid change facing update
    player.facing = Direction::Left;

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
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_eq!(
        timer.tags["ActivityComplete"],
        TagType::Duration(chrono::Duration::seconds(60))
    );

    assert_activity_started(60_000, ui::pane::PaneTitle::Logging, &mut update_rx);

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
    assert_xp_is_updated(player.id, Logging, 6, &mut update_rx);

    assert_activity_started(60_000, ui::pane::PaneTitle::Logging, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(1, Material, "Hardwood Log", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);

    assert_eq!(player.get_xp(Logging), exp_xp);
}

#[test]
fn can_chop_oak_tree() {
    let (
        mut player,
        mut map,
        mut obstacles,
        mut characters,
        mut item_class_specifiers,
        mut items,
        mut facilities,
        mut inventories,
        mut _rng,
        mut timer,
        update_tx,
        mut update_rx,
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(10, 11);

    let mut rng = Rng::new();

    // set facing to avoid change facing update
    player.facing = Direction::Left;

    player.endorse_with(":can_chop");

    let exp_xp = player.get_xp(Logging) + 8;

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
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_eq!(
        timer.tags["ActivityComplete"],
        TagType::Duration(chrono::Duration::seconds(60))
    );

    assert_activity_started(60_000, ui::pane::PaneTitle::Logging, &mut update_rx);

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
    assert_xp_is_updated(player.id, Logging, 8, &mut update_rx);

    assert_activity_started(60_000, ui::pane::PaneTitle::Logging, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(1, Material, "Hardwood Log", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);

    assert_eq!(player.get_xp(Logging), exp_xp);
}

#[test]
fn can_pick_apple_tree() {
    let (
        mut player,
        mut map,
        mut obstacles,
        mut characters,
        mut item_class_specifiers,
        mut items,
        mut facilities,
        mut inventories,
        mut _rng,
        mut timer,
        update_tx,
        mut update_rx,
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(10, 9);

    let mut rng = Rng::new();

    player.endorse_with(":can_pick");

    // set facing to avoid change facing update
    player.facing = Direction::Left;

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
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_eq!(
        timer.tags["ActivityComplete"],
        TagType::Duration(chrono::Duration::seconds(60))
    );

    assert_activity_started(60_000, ui::pane::PaneTitle::PickingApples, &mut update_rx);

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
    assert_xp_is_updated(player.id, Harvesting, 10, &mut update_rx);

    assert_activity_started(60_000, ui::pane::PaneTitle::PickingApples, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(1, Food, "Apple", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn can_pick_olive_tree() {
    let (
        mut player,
        mut map,
        mut obstacles,
        mut characters,
        mut item_class_specifiers,
        mut items,
        mut facilities,
        mut inventories,
        mut _rng,
        mut timer,
        update_tx,
        mut update_rx,
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(10, 10);

    let mut rng = Rng::new();

    player.endorse_with(":can_pick");

    // set facing to avoid change facing update
    player.facing = Direction::Left;

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
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_eq!(
        timer.tags["ActivityComplete"],
        TagType::Duration(chrono::Duration::seconds(90))
    );

    assert_activity_started(90_000, ui::pane::PaneTitle::PickingOlives, &mut update_rx);

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
    assert_xp_is_updated(player.id, Harvesting, 15, &mut update_rx);

    assert_activity_started(90_000, ui::pane::PaneTitle::PickingOlives, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(1, Food, "Olive", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn player_earns_10_xp_harvesting_for_picking_an_apple() {
    let (
        mut player,
        mut map,
        mut obstacles,
        mut characters,
        mut item_class_specifiers,
        mut items,
        mut facilities,
        mut inventories,
        mut _rng,
        mut timer,
        update_tx,
        mut update_rx,
        command_tx,
        _command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(10, 9);

    let mut rng = Rng::new();

    let exp_xp = player.get_xp(Harvesting) + 10;
    player.endorse_with(":can_pick");

    // set facing to avoid change facing update
    player.facing = Direction::Left;

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
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_eq!(
        timer.tags["ActivityComplete"],
        TagType::Duration(chrono::Duration::seconds(60))
    );

    assert_activity_started(60_000, ui::pane::PaneTitle::PickingApples, &mut update_rx);

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

    assert_eq!(player.get_xp(Harvesting), exp_xp);
}

#[test]
fn player_earns_15_xp_harvesting_for_picking_an_olive() {
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
    ) = initialize_game_system_with_player_at(10, 10);

    player.endorse_with(":can_pick");
    give_player_level(Harvesting, 10, &mut player);

    // set facing to avoid change facing update
    player.facing = Direction::Left;
    let exp_xp = player.get_xp(Harvesting) + 15;

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
        &Command::Move(Direction::Left, MoveCommandMode::Use),
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

    assert_eq!(player.get_xp(Harvesting), exp_xp);
}

#[test]
fn skilltime_is_accounted_for_while_picking_apples() {
    let (
        mut player,
        mut map,
        mut obstacles,
        mut characters,
        mut item_class_specifiers,
        mut items,
        mut facilities,
        mut inventories,
        mut _rng,
        mut timer,
        update_tx,
        mut update_rx,
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(10, 9);

    let mut rng = Rng::new();

    player.endorse_with(":can_pick");
    give_player_level(Harvesting, 12, &mut player);

    // set facing to avoid change facing update
    player.facing = Direction::Left;

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
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_eq!(
        timer.tags["ActivityComplete"],
        TagType::Duration(chrono::Duration::seconds(49))
    );

    assert_activity_started(49_000, ui::pane::PaneTitle::PickingApples, &mut update_rx);

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
    assert_xp_is_updated(player.id, Harvesting, 10, &mut update_rx);

    assert_activity_started(49_000, ui::pane::PaneTitle::PickingApples, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(1, Food, "Apple", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn skilltime_is_accounted_for_while_picking_olives() {
    let (
        mut player,
        mut map,
        mut obstacles,
        mut characters,
        mut item_class_specifiers,
        mut items,
        mut facilities,
        mut inventories,
        mut _rng,
        mut timer,
        update_tx,
        mut update_rx,
        command_tx,
        mut command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(10, 10);

    let mut rng = Rng::new();

    player.endorse_with(":can_pick");
    give_player_level(Harvesting, 20, &mut player);

    // set facing to avoid change facing update
    player.facing = Direction::Left;

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
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_eq!(
        timer.tags["ActivityComplete"],
        TagType::Duration(chrono::Duration::seconds(71))
    );

    assert_activity_started(71_000, ui::pane::PaneTitle::PickingOlives, &mut update_rx);

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
    assert_xp_is_updated(player.id, Harvesting, 15, &mut update_rx);

    assert_activity_started(71_000, ui::pane::PaneTitle::PickingOlives, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_spawning_item(1, Food, "Olive", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn a_timer_is_set_for_fruit_and_wood_regeneration() {
    let (
        _player,
        map,
        _obstacles,
        _characters,
        _item_class_specifiers,
        _items,
        _facilities,
        _inventories,
        _rng,
        timer,
        _update_tx,
        _update_rx,
        _command_tx,
        _command_rx,
        _game_state,
    ) = initialize_game_system_with_player_at(10, 10);

    let facility_id = get_facility_id_at(9, 8, &map);

    let tag = format!("regeneration for {:?}", facility_id);
    assert_eq!(timer.tags[&tag], TagType::Ticks(54000))
}

#[test]
fn wood_regenerates_upon_maintenance() {
    let (
        _player,
        map,
        _obstacles,
        _characters,
        _item_class_specifiers,
        _items,
        mut facilities,
        _inventories,
        _rng,
        _timer,
        _update_tx,
        _update_rx,
        _command_tx,
        _command_rx,
        _game_state,
    ) = initialize_game_system_with_player_at(10, 10);

    let facility_id = get_facility_id_at(9, 8, &map);
    let facility = facilities.get_mut(facility_id).expect("unable to get tree");

    let exp_logs = facility.get_property("logs") + 1;

    facility.maintenance();

    assert_eq!(facility.get_property("logs"), exp_logs);
}

#[test]
fn fruit_regenerates_upon_maintenance() {
    let (
        _player,
        map,
        _obstacles,
        _characters,
        _item_class_specifiers,
        _items,
        mut facilities,
        _inventories,
        _rng,
        _timer,
        _update_tx,
        _update_rx,
        _command_tx,
        _command_rx,
        _game_state,
    ) = initialize_game_system_with_player_at(10, 10);

    let facility_id = get_facility_id_at(9, 9, &map);
    let facility = facilities.get_mut(facility_id).expect("unable to get tree");

    let exp_fruit = facility.get_property("fruit") + 1;

    facility.maintenance();

    assert_eq!(facility.get_property("fruit"), exp_fruit);
}
