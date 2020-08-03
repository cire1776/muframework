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

    assert_eq!(timer.tags["ActivityComplete"], TagType::Ticks(291 * 60));

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

    assert_eq!(timer.tags["ActivityComplete"], TagType::Ticks(583 * 60));

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

    assert_eq!(timer.tags["ActivityComplete"], TagType::Ticks(1169 * 60));

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

#[test]
fn spawn_facility_creates_a_new_construction_site() {
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
    ) = initialize_game_system_with_player_at(3, 7);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Small Construction Site".into(),
            "property: size => 1".into(),
        ),
        Some(&update_tx),
        Some(command_tx),
    );

    let new_facility = get_facility_at(3, 7, &map, &mut facilities);

    assert_eq!(new_facility.class, FacilityClass::ConstructionSite);
    assert_eq!(new_facility.get_property("size"), 1);
}

#[test]
fn spawn_facility_properly_blocks_at_new_facility() {
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
    ) = initialize_game_system_with_player_at(3, 7);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Small Construction Site".into(),
            "property: size => 1".into(),
        ),
        Some(&update_tx),
        Some(command_tx),
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
        &mut game_data,
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Right, MoveCommandMode::Normal),
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
        &mut game_data,
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Left, MoveCommandMode::Normal),
        None,
        None,
    );

    assert_eq!(player.x, 4);
    assert_eq!(player.y, 7);
}

#[test]
fn small_construction_site_returns_does_not_respond_below_level_10() {
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
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 9, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Small Construction Site".into(),
            "property: size => 1".into(),
        ),
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
        &mut game_data,
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        Some(&update_tx),
        Some(command_tx),
    );

    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx)
}

#[test]
fn medium_construction_site_returns_does_not_respond_below_level_18() {
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
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 17, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Small Construction Site".into(),
            "property: size => 2".into(),
        ),
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
        &mut game_data,
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        Some(&update_tx),
        Some(command_tx),
    );

    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx)
}

#[test]
fn large_construction_site_returns_does_not_respond_below_level_32() {
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
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 31, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Small Construction Site".into(),
            "property: size => 3".into(),
        ),
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
        &mut game_data,
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        Some(&update_tx),
        Some(command_tx),
    );

    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx)
}

#[test]
fn small_construction_site_presents_firepit_as_option_for_level_10() {
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
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 10, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Small Construction Site".into(),
            "property: size => 1".into(),
        ),
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
        &mut game_data,
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        Some(&update_tx),
        Some(command_tx),
    );

    assert_is_display_options(
        vec!["Firepit"],
        ActionContinuation::SetConstructionSite,
        &mut update_rx,
    );
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx)
}

#[test]
fn small_construction_site_presents_firepit_and_lumbermill_as_options_for_level_12() {
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
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 12, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Small Construction Site".into(),
            "property: size => 1".into(),
        ),
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
        &mut game_data,
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        Some(&update_tx),
        Some(command_tx),
    );

    assert_is_display_options(
        vec!["Firepit", "Lumbermill"],
        ActionContinuation::SetConstructionSite,
        &mut update_rx,
    );
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx)
}

#[test]
fn small_construction_site_presents_firepit_and_lumbermill_and_chest_as_options_for_level_13() {
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
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 13, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Small Construction Site".into(),
            "property: size => 1".into(),
        ),
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
        &mut game_data,
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        Some(&update_tx),
        Some(command_tx),
    );

    assert_is_display_options(
        vec!["Firepit", "Lumbermill", "Chest"],
        ActionContinuation::SetConstructionSite,
        &mut update_rx,
    );
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx)
}

#[test]
fn small_construction_site_presents_firepit_lumbermill_chest_and_well_as_options_for_level_14() {
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
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 14, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Small Construction Site".into(),
            "property: size => 1".into(),
        ),
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
        &mut game_data,
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        Some(&update_tx),
        Some(command_tx),
    );

    assert_is_display_options(
        vec!["Firepit", "Lumbermill", "Chest", "Well"],
        ActionContinuation::SetConstructionSite,
        &mut update_rx,
    );
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx)
}

#[test]
fn medium_construction_site_presents_fruit_press_as_option_for_level_18() {
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
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 18, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Construction Site".into(),
            "property: size => 2".into(),
        ),
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
        &mut game_data,
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        Some(&update_tx),
        Some(command_tx),
    );

    assert_is_display_options(
        vec!["Fruit Press"],
        ActionContinuation::SetConstructionSite,
        &mut update_rx,
    );
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx)
}

#[test]
fn small_construction_site_can_be_set_to_build_firepit_upon_continuation() {
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
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 10, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Construction Site".into(),
            "property: size => 1".into(),
        ),
        None,
        None,
    );

    let site_id = get_facility_id_at(3, 7, &map);

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
        &Command::ChoiceSelected(1, ActionContinuation::SetConstructionSite, site_id),
        Some(&update_tx),
        Some(command_tx),
    );

    let mut facilities2 = facilities.clone();
    let site = get_facility_at(3, 7, &map, &mut facilities2);

    assert_eq!(site.get_property("blueprint"), 1);

    assert_eq!(site.get_property("hardwood"), 0);
    assert_eq!(site.get_property("softwood"), 0);
    assert_eq!(site.get_property("stone"), 16);
    assert_eq!(site.get_property("rope"), 0);
}

#[test]
fn small_construction_site_can_be_set_to_build_lumbermill_upon_continuation() {
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
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 12, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Construction Site".into(),
            "property: size => 1".into(),
        ),
        None,
        None,
    );

    let site_id = get_facility_id_at(3, 7, &map);

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
        &Command::ChoiceSelected(2, ActionContinuation::SetConstructionSite, site_id),
        Some(&update_tx),
        Some(command_tx),
    );

    let mut facilities2 = facilities.clone();
    let site = get_facility_at(3, 7, &map, &mut facilities2);

    assert_eq!(site.get_property("blueprint"), 2);

    assert_eq!(site.get_property("hardwood"), 0);
    assert_eq!(site.get_property("softwood"), 64);
    assert_eq!(site.get_property("stone"), 16);
    assert_eq!(site.get_property("rope"), 0);
}

#[test]
fn small_construction_site_can_be_set_to_build_chest_upon_continuation() {
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
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 13, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Construction Site".into(),
            "property: size => 1".into(),
        ),
        None,
        None,
    );

    let site_id = get_facility_id_at(3, 7, &map);

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
        &Command::ChoiceSelected(3, ActionContinuation::SetConstructionSite, site_id),
        Some(&update_tx),
        Some(command_tx),
    );

    let mut facilities2 = facilities.clone();
    let site = get_facility_at(3, 7, &map, &mut facilities2);

    assert_eq!(site.get_property("blueprint"), 3);

    assert_eq!(site.get_property("hardwood"), 128);
    assert_eq!(site.get_property("softwood"), 0);
    assert_eq!(site.get_property("stone"), 0);
    assert_eq!(site.get_property("rope"), 0);
}

#[test]
fn small_construction_site_can_be_set_to_build_well_upon_continuation() {
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
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 14, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Construction Site".into(),
            "property: size => 1".into(),
        ),
        None,
        None,
    );

    let site_id = get_facility_id_at(3, 7, &map);

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
        &Command::ChoiceSelected(4, ActionContinuation::SetConstructionSite, site_id),
        Some(&update_tx),
        Some(command_tx),
    );

    let mut facilities2 = facilities.clone();
    let site = get_facility_at(3, 7, &map, &mut facilities2);

    assert_eq!(site.get_property("blueprint"), 4);

    assert_eq!(site.get_property("hardwood"), 0);
    assert_eq!(site.get_property("softwood"), 64);
    assert_eq!(site.get_property("stone"), 64);
    assert_eq!(site.get_property("rope"), 0);
}

#[test]
fn once_set_construction_site_cannot_be_reset() {
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
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 14, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Construction Site".into(),
            "property: size => 1".into(),
        ),
        None,
        None,
    );

    let site_id = get_facility_id_at(3, 7, &map);

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
        &Command::ChoiceSelected(4, ActionContinuation::SetConstructionSite, site_id),
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
        &mut game_data,
        &mut rng,
        &mut timer,
        None,
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        Some(&update_tx),
        Some(command_tx),
    );

    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn small_construction_site_starts_timer_to_add_stones_to_build_firepit() {
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
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 14, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Construction Site".into(),
            "property: size => 1".into(),
        ),
        None,
        None,
    );

    let site_id = get_facility_id_at(3, 7, &map);

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
        &Command::ChoiceSelected(1, ActionContinuation::SetConstructionSite, site_id),
        None,
        None,
    );

    player.endorse_component_with(":wants_to_add", "stone");
    equip_player_with_spawned_item(Ore, "Stone", &mut player, &mut inventories, &mut items);
    give_player_spawned_items(Ore, "Stone", 15, &mut player, &mut inventories, &mut items);

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
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());

    assert_eq!(timer.tags["ActivityComplete"], TagType::Ticks(47 * 60));

    assert_activity_started(47000, ui::pane::PaneTitle::Building, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn small_construction_site_removes_one_stone_from_site_upon_timer_completion() {
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
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 10, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Construction Site".into(),
            "property: size => 1".into(),
        ),
        None,
        None,
    );

    let site_id = get_facility_id_at(3, 7, &map);

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
        &Command::ChoiceSelected(1, ActionContinuation::SetConstructionSite, site_id),
        None,
        None,
    );

    let exp_stones = {
        let site = get_facility_at(3, 7, &map, &mut facilities);
        site.get_property("stone") - 1
    };

    player.endorse_component_with(":wants_to_add", "stone");
    equip_player_with_spawned_item(Ore, "Stone", &mut player, &mut inventories, &mut items);
    give_player_spawned_items(Ore, "Stone", 15, &mut player, &mut inventories, &mut items);

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
        &Command::Move(Direction::Left, MoveCommandMode::Use),
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
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());

    let site = get_facility_at(3, 7, &map, &mut facilities);
    assert_eq!(site.get_property("stone"), exp_stones);
}

#[test]
fn small_construction_site_builds_firepit_upon_last_timer_completion() {
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
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 10, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Construction Site".into(),
            "property: size => 1".into(),
        ),
        None,
        None,
    );

    let site_id = get_facility_id_at(3, 7, &map);

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
        &Command::ChoiceSelected(1, ActionContinuation::SetConstructionSite, site_id),
        None,
        None,
    );

    {
        let site = get_facility_at(3, 7, &map, &mut facilities);
        site.set_property("stone", 1);
    };

    player.endorse_component_with(":wants_to_add", "stone");
    equip_player_with_spawned_item(Ore, "Stone", &mut player, &mut inventories, &mut items);
    give_player_spawned_items(Ore, "Stone", 15, &mut player, &mut inventories, &mut items);

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
        &Command::Move(Direction::Left, MoveCommandMode::Use),
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
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());

    assert_activity_expired(&mut update_rx);
    assert_xp_is_updated(player.id, Construction, 5, &mut update_rx);
    assert_is_facility_updated(
        site_id,
        "A Firepit".into(),
        FacilityClass::Firepit,
        0,
        &mut update_rx,
    );
    assert_activity_started(51000, ui::PaneTitle::Building, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_is_activity_abort(&mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn small_construction_site_starts_timer_to_add_stones_to_build_lumbermill() {
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
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 12, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Construction Site".into(),
            "property: size => 1".into(),
        ),
        None,
        None,
    );

    let site_id = get_facility_id_at(3, 7, &map);

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
        &Command::ChoiceSelected(2, ActionContinuation::SetConstructionSite, site_id),
        None,
        None,
    );

    player.endorse_component_with(":wants_to_add", "stone");
    equip_player_with_spawned_item(Ore, "Stone", &mut player, &mut inventories, &mut items);
    give_player_spawned_items(Ore, "Stone", 15, &mut player, &mut inventories, &mut items);

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
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());

    assert_eq!(timer.tags["ActivityComplete"], TagType::Ticks(49 * 60));

    assert_activity_started(49000, ui::pane::PaneTitle::Building, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn small_construction_site_reduces_one_softwood_from_site_upon_timer_completion() {
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
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 12, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Construction Site".into(),
            "property: size => 1".into(),
        ),
        None,
        None,
    );

    let site_id = get_facility_id_at(3, 7, &map);

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
        &Command::ChoiceSelected(2, ActionContinuation::SetConstructionSite, site_id),
        None,
        None,
    );

    let exp_softwood = {
        let site = get_facility_at(3, 7, &map, &mut facilities);
        site.get_property("softwood") - 1
    };

    player.endorse_component_with(":wants_to_add", "softwood");
    equip_player_with_spawned_item(
        Material,
        "Softwood Plank",
        &mut player,
        &mut inventories,
        &mut items,
    );
    give_player_spawned_items(
        Material,
        "Softwood Plank",
        15,
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
        &Command::Move(Direction::Left, MoveCommandMode::Use),
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
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());

    let site = get_facility_at(3, 7, &map, &mut facilities);
    assert_eq!(site.get_property("softwood"), exp_softwood);
}

#[test]
fn small_construction_site_builds_lumbermill_upon_last_ingredient_added() {
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
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 12, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Construction Site".into(),
            "property: size => 1".into(),
        ),
        None,
        None,
    );

    let site_id = get_facility_id_at(3, 7, &map);

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
        &Command::ChoiceSelected(2, ActionContinuation::SetConstructionSite, site_id),
        None,
        None,
    );

    {
        let site = get_facility_at(3, 7, &map, &mut facilities);
        site.set_property("stone", 1);
        site.set_property("softwood", 0);
    };

    player.endorse_component_with(":wants_to_add", "stone");
    equip_player_with_spawned_item(Ore, "Stone", &mut player, &mut inventories, &mut items);
    give_player_spawned_items(Ore, "Stone", 15, &mut player, &mut inventories, &mut items);

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
        &Command::Move(Direction::Left, MoveCommandMode::Use),
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
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());

    assert_activity_expired(&mut update_rx);
    assert_xp_is_updated(player.id, Construction, 5, &mut update_rx);
    assert_is_facility_updated(
        site_id,
        "A Simple Lumbermill".into(),
        FacilityClass::Lumbermill,
        0,
        &mut update_rx,
    );
    assert_activity_started(49000, ui::PaneTitle::Building, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_is_activity_abort(&mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn small_construction_site_builds_lumbermill_with_properties_set() {
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
        _update_tx,
        _update_rx,
        _command_tx,
        _command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 12, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Construction Site".into(),
            "property: size => 1".into(),
        ),
        None,
        None,
    );

    let site_id = get_facility_id_at(3, 7, &map);

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
        &Command::ChoiceSelected(2, ActionContinuation::SetConstructionSite, site_id),
        None,
        None,
    );

    {
        let site = get_facility_at(3, 7, &map, &mut facilities);
        site.set_property("stone", 1);
        site.set_property("softwood", 0);
    };

    player.endorse_component_with(":wants_to_add", "stone");
    equip_player_with_spawned_item(Ore, "Stone", &mut player, &mut inventories, &mut items);
    give_player_spawned_items(Ore, "Stone", 15, &mut player, &mut inventories, &mut items);

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
        &Command::Move(Direction::Left, MoveCommandMode::Use),
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
        &mut game_data,
        &mut rng,
        &mut timer,
        activity,
        &Command::ActivityComplete,
        None,
        None,
    );

    let lumbermill = get_facility_at(3, 7, &map, &mut facilities);

    assert_ne!(lumbermill.get_property("chance_of_breakage"), 0);
}

#[test]
fn small_construction_site_starts_timer_to_add_hardwood_to_build_chest() {
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
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 13, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Construction Site".into(),
            "property: size => 1".into(),
        ),
        None,
        None,
    );

    let site_id = get_facility_id_at(3, 7, &map);

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
        &Command::ChoiceSelected(3, ActionContinuation::SetConstructionSite, site_id),
        None,
        None,
    );

    player.endorse_component_with(":wants_to_add", "hardwood");
    equip_player_with_spawned_item(
        Material,
        "Hardwood Plank",
        &mut player,
        &mut inventories,
        &mut items,
    );
    give_player_spawned_items(
        Material,
        "Hardwood Plank",
        15,
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
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());

    assert_eq!(timer.tags["ActivityComplete"], TagType::Ticks(48 * 60));

    assert_activity_started(48000, ui::pane::PaneTitle::Building, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn small_construction_site_reduces_one_hardwood_from_site_upon_timer_completion() {
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
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 13, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Construction Site".into(),
            "property: size => 1".into(),
        ),
        None,
        None,
    );

    let site_id = get_facility_id_at(3, 7, &map);

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
        &Command::ChoiceSelected(3, ActionContinuation::SetConstructionSite, site_id),
        None,
        None,
    );

    let exp_softwood = {
        let site = get_facility_at(3, 7, &map, &mut facilities);
        site.get_property("hardwood") - 1
    };

    player.endorse_component_with(":wants_to_add", "hardwood");
    equip_player_with_spawned_item(
        Material,
        "Hardwood Plank",
        &mut player,
        &mut inventories,
        &mut items,
    );
    give_player_spawned_items(
        Material,
        "Hardwood Plank",
        15,
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
        &Command::Move(Direction::Left, MoveCommandMode::Use),
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
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());

    let site = get_facility_at(3, 7, &map, &mut facilities);
    assert_eq!(site.get_property("hardwood"), exp_softwood);
}

#[test]
fn small_construction_site_builds_chest_upon_last_ingredient_added() {
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
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 13, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Construction Site".into(),
            "property: size => 1".into(),
        ),
        None,
        None,
    );

    let site_id = get_facility_id_at(3, 7, &map);

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
        &Command::ChoiceSelected(3, ActionContinuation::SetConstructionSite, site_id),
        None,
        None,
    );

    {
        let site = get_facility_at(3, 7, &map, &mut facilities);
        site.set_property("hardwood", 1);
    };

    player.endorse_component_with(":wants_to_add", "hardwood");
    equip_player_with_spawned_item(
        Material,
        "Hardwood Plank",
        &mut player,
        &mut inventories,
        &mut items,
    );
    give_player_spawned_items(
        Material,
        "Hardwood Plank",
        15,
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
        &Command::Move(Direction::Left, MoveCommandMode::Use),
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
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());

    assert_activity_expired(&mut update_rx);
    assert_xp_is_updated(player.id, Construction, 5, &mut update_rx);
    assert_is_facility_updated(
        site_id,
        "A Chest".into(),
        FacilityClass::ClosedChest,
        0,
        &mut update_rx,
    );
    assert_activity_started(48000, ui::PaneTitle::Building, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_is_activity_abort(&mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn small_construction_site_starts_timer_to_add_stones_to_build_chest() {
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
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 14, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Construction Site".into(),
            "property: size => 1".into(),
        ),
        None,
        None,
    );

    let site_id = get_facility_id_at(3, 7, &map);

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
        &Command::ChoiceSelected(4, ActionContinuation::SetConstructionSite, site_id),
        None,
        None,
    );

    player.endorse_component_with(":wants_to_add", "softwood");
    equip_player_with_spawned_item(
        Material,
        "Softwood Plank",
        &mut player,
        &mut inventories,
        &mut items,
    );
    give_player_spawned_items(
        Material,
        "Softwood Plank",
        15,
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
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());

    assert_eq!(timer.tags["ActivityComplete"], TagType::Ticks(47 * 60));

    assert_activity_started(47000, ui::pane::PaneTitle::Building, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn small_construction_site_reduces_one_softwood_from_well_site_upon_timer_completion() {
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
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 14, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Construction Site".into(),
            "property: size => 1".into(),
        ),
        None,
        None,
    );

    let site_id = get_facility_id_at(3, 7, &map);

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
        &Command::ChoiceSelected(4, ActionContinuation::SetConstructionSite, site_id),
        None,
        None,
    );

    let exp_softwood = {
        let site = get_facility_at(3, 7, &map, &mut facilities);
        site.get_property("softwood") - 1
    };

    player.endorse_component_with(":wants_to_add", "softwood");
    equip_player_with_spawned_item(
        Material,
        "Softwood Plank",
        &mut player,
        &mut inventories,
        &mut items,
    );
    give_player_spawned_items(
        Material,
        "Softwood Plank",
        15,
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
        &Command::Move(Direction::Left, MoveCommandMode::Use),
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
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());

    let site = get_facility_at(3, 7, &map, &mut facilities);
    assert_eq!(site.get_property("softwood"), exp_softwood);
}

#[test]
fn small_construction_site_builds_dry_well_upon_last_ingredient_added() {
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
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 14, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Construction Site".into(),
            "property: size => 1".into(),
        ),
        None,
        None,
    );

    let site_id = get_facility_id_at(3, 7, &map);

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
        &Command::ChoiceSelected(4, ActionContinuation::SetConstructionSite, site_id),
        None,
        None,
    );

    {
        let site = get_facility_at(3, 7, &map, &mut facilities);
        site.set_property("softwood", 1);
        site.set_property("stone", 0);
    };

    player.endorse_component_with(":wants_to_add", "softwood");
    equip_player_with_spawned_item(
        Material,
        "Softwood Plank",
        &mut player,
        &mut inventories,
        &mut items,
    );
    give_player_spawned_items(
        Material,
        "Softwood Plank",
        15,
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
        &Command::Move(Direction::Left, MoveCommandMode::Use),
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
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_some());

    assert_activity_expired(&mut update_rx);
    assert_xp_is_updated(player.id, Construction, 5, &mut update_rx);
    assert_is_facility_updated(
        site_id,
        "A Well".into(),
        FacilityClass::Well,
        0,
        &mut update_rx,
    );
    assert_activity_started(47000, ui::PaneTitle::Building, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_is_activity_abort(&mut command_rx);
    assert_is_refresh_inventory(&mut command_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn small_construction_site_builds_well_with_properties_set() {
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
        _update_tx,
        _update_rx,
        _command_tx,
        _command_rx,
        mut game_state,
    ) = initialize_game_system_with_player_at(4, 7);

    player.facing = Direction::Left;
    give_player_level(Construction, 14, &mut player);

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
        &Command::SpawnFacility(
            3,
            7,
            FacilityClass::ConstructionSite,
            "A Construction Site".into(),
            "property: size => 1".into(),
        ),
        None,
        None,
    );

    let site_id = get_facility_id_at(3, 7, &map);

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
        &Command::ChoiceSelected(4, ActionContinuation::SetConstructionSite, site_id),
        None,
        None,
    );

    {
        let site = get_facility_at(3, 7, &map, &mut facilities);
        site.set_property("stone", 1);
        site.set_property("softwood", 0);
    };

    player.endorse_component_with(":wants_to_add", "stone");
    equip_player_with_spawned_item(Ore, "Stone", &mut player, &mut inventories, &mut items);
    give_player_spawned_items(Ore, "Stone", 15, &mut player, &mut inventories, &mut items);

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
        &Command::Move(Direction::Left, MoveCommandMode::Use),
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
        &mut game_data,
        &mut rng,
        &mut timer,
        activity,
        &Command::ActivityComplete,
        None,
        None,
    );

    let well = get_facility_at(3, 7, &map, &mut facilities);

    assert_eq!(well.get_property("fluid"), 0);
    assert_eq!(well.get_property("depth"), 0);
    assert_eq!(well.get_property("chance_of_hitting_oil"), 10000);
    assert_eq!(well.get_property("chance_of_hitting_water"), 100);
    assert_eq!(well.get_property("chance_of_hitting_bedrock"), 50000);
    assert_eq!(well.get_property("chance_of_drying_up"), 5000);
}
