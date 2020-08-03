use super::*;
use common::timer::TagType;

#[test]
fn can_equip_a_book_in_the_on_hand() {
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
        _command_tx,
        _command_rx,
        mut game_state,
    ) = initialize_game_system();

    let book = give_player_spawned_items(
        Book,
        "Summer of the Blue",
        1,
        &mut player,
        &mut inventories,
        &mut items,
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
        &Command::EquipItem(book.id),
        Some(&update_tx),
        None,
    );

    assert_eq!(
        player.mounting_points.at(&MountingPoint::OnHand),
        Some(book.id)
    );
}

#[test]
#[should_panic(expected = "unable to find mounted book.")]
fn cannot_read_an_unmounted_book() {
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
    ) = initialize_game_system();

    give_player_spawned_items(
        Book,
        "Summer of the Blue",
        1,
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
        &Command::UseItem,
        Some(&update_tx),
        Some(command_tx),
    );

    assert!(activity.is_none());
}

#[test]
fn can_read_a_book() {
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
    ) = initialize_game_system();

    rng.set_fail("levelling check");

    player.endorse_with(":can_use");
    give_player_level(Intellectual, 5, &mut player);

    equip_player_with_spawned_item(
        Book,
        "Summer of the Blue",
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
        &Command::UseItem,
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    assert!(activity.is_some());

    assert_eq!(timer.tags["ActivityComplete"], TagType::Ticks(56 * 60));

    assert_activity_started(56_000, Reading, &mut update_rx);
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
        Some(command_tx),
    );

    assert_activity_expired(&mut update_rx);
    assert_xp_is_updated(player.id, Intellectual, 10, &mut update_rx);
    assert_xp_is_updated(player.id, Cultural, 10, &mut update_rx);
    assert_activity_started(56_000, Reading, &mut update_rx);

    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn reading_a_book_earns_10_intellectual_xp() {
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
    ) = initialize_game_system();

    rng.set_fail("levelling check");

    player.endorse_with(":can_use");
    let exp_xp = player.get_xp(Intellectual) + 10;
    give_player_level(Intellectual, 5, &mut player);

    equip_player_with_spawned_item(
        Book,
        "Summer of the Blue",
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
        &Command::UseItem,
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    assert!(activity.is_some());

    assert_eq!(timer.tags["ActivityComplete"], TagType::Ticks(56 * 60));

    assert_activity_started(56_000, Reading, &mut update_rx);
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
        Some(command_tx),
    );

    assert_eq!(player.get_xp(Intellectual), exp_xp);
}

#[test]
fn reading_a_book_with_an_associated_skill_earns_10_points_for_that_skill() {
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
    ) = initialize_game_system();

    rng.set_fail("levelling check");

    player.endorse_with(":can_use");
    let exp_xp = player.get_xp(Mining) + 10;
    give_player_level(Intellectual, 1, &mut player);

    equip_player_with_spawned_item(Book, "Mines!", &mut player, &mut inventories, &mut items);

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
        &Command::UseItem,
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    assert!(activity.is_some());

    assert_eq!(timer.tags["ActivityComplete"], TagType::Ticks(60 * 60));

    assert_activity_started(60_000, Reading, &mut update_rx);
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
        Some(command_tx),
    );

    assert_eq!(player.get_xp(Mining), exp_xp);
}

#[test]
fn skilltime_is_taken_into_account() {
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
    ) = initialize_game_system();

    player.endorse_with(":can_use");

    equip_player_with_spawned_item(Book, "Mines!", &mut player, &mut inventories, &mut items);
    give_player_level(Intellectual, 10, &mut player);

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
        &Command::UseItem,
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    assert!(activity.is_some());

    assert_eq!(timer.tags["ActivityComplete"], TagType::Ticks(51 * 60));

    assert_activity_started(51_000, Reading, &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn if_a_player_does_not_have_the_minimum_intellectual_level_they_cannot_read_the_book() {
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
    ) = initialize_game_system();

    player.endorse_with(":can_use");
    give_player_level(Intellectual, 0, &mut player);

    equip_player_with_spawned_item(Book, "Mines!", &mut player, &mut inventories, &mut items);

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
        &Command::UseItem,
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    assert!(activity.is_none());
}

#[test]
fn if_player_is_too_intellectual_for_book_they_get_no_intellectual_experience() {
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
    ) = initialize_game_system();

    player.endorse_with(":can_use");
    give_player_level(Intellectual, 45, &mut player);

    let exp_xp = player.get_xp(Intellectual) + 0;

    equip_player_with_spawned_item(
        Book,
        "Summer of the Blue",
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
        &Command::UseItem,
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    assert!(activity.is_some());

    assert_activity_started(16_000, Reading, &mut update_rx);
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
        Some(command_tx),
    );

    assert_eq!(player.get_xp(Intellectual), exp_xp);
}

#[test]
fn a_player_must_have_the_minimum_associated_skill_to_earn_xp_for_that_skill() {
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
    ) = initialize_game_system();

    rng.set_fail("levelling check");

    player.endorse_with(":can_use");
    let exp_xp = player.get_xp(Engineering);
    give_player_level(Intellectual, 1, &mut player);
    give_player_level(Engineering, 1, &mut player);

    equip_player_with_spawned_item(
        Book,
        "The Compleat Engineer",
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
        &Command::UseItem,
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    assert!(activity.is_some());

    assert_eq!(timer.tags["ActivityComplete"], TagType::Ticks(60 * 60));

    assert_activity_started(60_000, Reading, &mut update_rx);
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
        Some(command_tx),
    );

    assert_eq!(player.get_xp(Engineering), exp_xp);
}

#[test]
fn if_player_has_too_high_topic_skill_they_earn_no_topic_xp() {
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
    ) = initialize_game_system();

    rng.set_fail("levelling check");

    player.endorse_with(":can_use");
    let exp_xp = player.get_xp(Engineering);
    give_player_level(Intellectual, 1, &mut player);
    give_player_level(Engineering, 45, &mut player);

    equip_player_with_spawned_item(
        Book,
        "The Compleat Engineer",
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
        &Command::UseItem,
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    assert!(activity.is_some());

    assert_eq!(timer.tags["ActivityComplete"], TagType::Ticks(60 * 60));

    assert_activity_started(60_000, Reading, &mut update_rx);
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
        Some(command_tx),
    );

    assert_eq!(player.get_xp(Engineering), exp_xp);
}
