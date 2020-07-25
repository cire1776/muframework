use super::*;

use common::timer::TagType;
use ui::PaneTitle;

#[test]
fn starting_build_without_supplies_sends_message() {
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
    ) = initialize_game_system_with_player_at(3, 7);

    clear_inventory(&mut player, &mut inventories, &mut items);

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
        &Command::ConstructionSiteBegin,
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_none());

    assert_is_message(
        "Insufficient supplies to build any site.",
        MessageType::System,
        &mut update_rx,
    );

    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn starting_build_with_supplies_for_small_site_sends_1_option() {
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
    ) = initialize_game_system_with_player_at(3, 7);

    clear_inventory(&mut player, &mut inventories, &mut items);

    give_player_spawned_items(
        Material,
        "Softwood Plank",
        20,
        &mut player,
        &mut inventories,
        &mut items,
    );
    give_player_spawned_items(Ore, "Stone", 5, &mut player, &mut inventories, &mut items);

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
        &Command::ConstructionSiteBegin,
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_none());

    assert_is_display_options(
        ["Small Construction Site"].to_vec(),
        ActionContinuation::ConstructionSite,
        &mut update_rx,
    );

    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn starting_build_with_supplies_for_medium_site_sends_2_options() {
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
    ) = initialize_game_system_with_player_at(3, 7);

    clear_inventory(&mut player, &mut inventories, &mut items);

    give_player_spawned_items(
        Material,
        "Softwood Plank",
        64,
        &mut player,
        &mut inventories,
        &mut items,
    );
    give_player_spawned_items(Ore, "Stone", 32, &mut player, &mut inventories, &mut items);
    give_player_spawned_items(
        Material,
        "Rope",
        16,
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
        &Command::ConstructionSiteBegin,
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_none());

    assert_is_display_options(
        ["Small Construction Site", "Medium Construction Site"].to_vec(),
        ActionContinuation::ConstructionSite,
        &mut update_rx,
    );

    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn starting_build_with_supplies_for_large_site_sends_all_options() {
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
    ) = initialize_game_system_with_player_at(3, 7);

    clear_inventory(&mut player, &mut inventories, &mut items);

    give_player_spawned_items(
        Material,
        "Softwood Plank",
        128,
        &mut player,
        &mut inventories,
        &mut items,
    );
    give_player_spawned_items(Ore, "Stone", 64, &mut player, &mut inventories, &mut items);
    give_player_spawned_items(
        Material,
        "Rope",
        64,
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
        &Command::ConstructionSiteBegin,
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_none());

    assert_is_display_options(
        [
            "Small Construction Site",
            "Medium Construction Site",
            "Large Construction Site",
        ]
        .to_vec(),
        ActionContinuation::ConstructionSite,
        &mut update_rx,
    );

    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn can_complete_a_small_construction_site() {
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
    ) = initialize_game_system_with_player_at(3, 7);

    clear_inventory(&mut player, &mut inventories, &mut items);

    give_player_level(Construction, 10, &mut player);

    give_player_spawned_items(
        Material,
        "Softwood Plank",
        128,
        &mut player,
        &mut inventories,
        &mut items,
    );
    give_player_spawned_items(Ore, "Stone", 64, &mut player, &mut inventories, &mut items);
    give_player_spawned_items(
        Material,
        "Rope",
        64,
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
        &Command::ChoiceSelected(1, ActionContinuation::ConstructionSite, 0),
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    assert!(activity.is_some());

    assert_eq!(
        timer.tags["ActivityComplete"],
        TagType::Duration(chrono::Duration::seconds(291))
    );

    assert_activity_started(291000, PaneTitle::Building, &mut update_rx);
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
        &mut game_data,
        &mut rng,
        &mut timer,
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert_activity_expired(&mut update_rx);
    assert_xp_is_updated(player.id, Construction, 50, &mut update_rx);
    assert_activity_started(291000, PaneTitle::Building, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_activity_abort(&mut command_rx);
    assert_is_spawn_facility(
        3,
        7,
        FacilityClass::ConstructionSite,
        "A Small Construction Site",
        "property: size => 1",
        &mut command_rx,
    );
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn can_complete_a_medium_construction_site() {
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
    ) = initialize_game_system_with_player_at(3, 7);

    clear_inventory(&mut player, &mut inventories, &mut items);

    give_player_level(Construction, 18, &mut player);

    give_player_spawned_items(
        Material,
        "Softwood Plank",
        128,
        &mut player,
        &mut inventories,
        &mut items,
    );
    give_player_spawned_items(Ore, "Stone", 64, &mut player, &mut inventories, &mut items);
    give_player_spawned_items(
        Material,
        "Rope",
        64,
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
        &Command::ChoiceSelected(2, ActionContinuation::ConstructionSite, 0),
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    assert!(activity.is_some());

    assert_eq!(
        timer.tags["ActivityComplete"],
        TagType::Duration(chrono::Duration::seconds(583))
    );

    assert_activity_started(583000, PaneTitle::Building, &mut update_rx);
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
        &mut game_data,
        &mut rng,
        &mut timer,
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert_activity_expired(&mut update_rx);
    assert_xp_is_updated(player.id, Construction, 250, &mut update_rx);
    assert_activity_started(583000, PaneTitle::Building, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_activity_abort(&mut command_rx);
    assert_is_spawn_facility(
        3,
        7,
        FacilityClass::ConstructionSite,
        "A Construction Site",
        "property: size => 2",
        &mut command_rx,
    );
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn can_complete_a_large_construction_site() {
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
    ) = initialize_game_system_with_player_at(3, 7);

    clear_inventory(&mut player, &mut inventories, &mut items);

    give_player_level(Construction, 32, &mut player);

    give_player_spawned_items(
        Material,
        "Softwood Plank",
        128,
        &mut player,
        &mut inventories,
        &mut items,
    );
    give_player_spawned_items(Ore, "Stone", 64, &mut player, &mut inventories, &mut items);
    give_player_spawned_items(
        Material,
        "Rope",
        64,
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
        &Command::ChoiceSelected(3, ActionContinuation::ConstructionSite, 0),
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    assert!(activity.is_some());

    assert_eq!(
        timer.tags["ActivityComplete"],
        TagType::Duration(chrono::Duration::seconds(1169))
    );

    assert_activity_started(1169000, PaneTitle::Building, &mut update_rx);
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
        &mut game_data,
        &mut rng,
        &mut timer,
        activity,
        &Command::ActivityComplete,
        Some(&update_tx),
        Some(command_tx),
    );

    assert_activity_expired(&mut update_rx);
    assert_xp_is_updated(player.id, Construction, 1250, &mut update_rx);
    assert_activity_started(1169000, PaneTitle::Building, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);

    assert_is_activity_abort(&mut command_rx);
    assert_is_spawn_facility(
        3,
        7,
        FacilityClass::ConstructionSite,
        "A Large, Hectic Construction Site",
        "property: size => 3",
        &mut command_rx,
    );
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}
