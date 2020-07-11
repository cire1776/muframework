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
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Right, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_player_is_at(14, 1, &player);

    assert_eq!(
        timer.tags["ActivityComplete"],
        TagType::Duration(chrono::Duration::seconds(60))
    );
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
        &mut rng,
        &mut timer,
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());
    assert_activity_expired(&mut update_rx);
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
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Right, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_player_is_at(14, 1, &player);

    assert_eq!(
        timer.tags["ActivityComplete"],
        TagType::Duration(chrono::Duration::seconds(45))
    );
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
        &mut rng,
        &mut timer,
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());
    assert_activity_expired(&mut update_rx);
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
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Right, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );

    assert_player_is_at(14, 1, &player);

    assert_eq!(
        timer.tags["ActivityComplete"],
        TagType::Duration(chrono::Duration::seconds(45))
    );
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
        &mut rng,
        &mut timer,
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());
    assert_activity_expired(&mut update_rx);
    assert_activity_started(45_000, PaneTitle::NetFishing, &mut update_rx);
    assert_is_spawning_item(player.id, Ingredient, "Frog", &mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}
