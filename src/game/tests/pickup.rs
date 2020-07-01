use super::*;

#[test]
fn single_item_can_be_picked_up() {
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
    ) = initialize_game_system_with_player_at(8, 7);

    clear_inventory(&mut player, &mut inventories, &mut items);
    spawn_item_at(player.x, player.y, Ingredient, "Trout", 1, &mut items);

    let items_on_ground = get_items_at(player.x, player.y, &items);
    let target_item = items_on_ground[0].clone();
    let target_item_id = target_item.id;

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
        &Command::TakeItem(1),
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    assert!(activity.is_none());

    assert_item_removed(target_item_id, &mut update_rx);
    assert_inventory_updated(&mut vec![target_item], &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn first_item_in_bundle_can_be_picked_up() {
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
    ) = initialize_game_system_with_player_at(8, 7);

    clear_inventory(&mut player, &mut inventories, &mut items);

    spawn_item_at(player.x, player.y, Ingredient, "Trout", 1, &mut items);
    spawn_item_at(
        player.x,
        player.y,
        Material,
        "Hardwood Plank",
        1,
        &mut items,
    );
    spawn_item_at(
        player.x,
        player.y,
        Ingredient,
        "Bottle of Water",
        1,
        &mut items,
    );

    let items_on_ground = get_items_at(player.x, player.y, &items);
    let target_item = items_on_ground[0].clone();
    let target_item_id = target_item.id;

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
        &Command::TakeItem(1),
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    assert!(activity.is_none());

    assert_item_removed(target_item_id, &mut update_rx);
    assert_inventory_updated(&mut vec![target_item], &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn third_item_in_bundle_can_be_picked_up() {
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
    ) = initialize_game_system_with_player_at(8, 7);

    clear_inventory(&mut player, &mut inventories, &mut items);

    spawn_item_at(player.x, player.y, Ingredient, "Trout", 1, &mut items);
    spawn_item_at(
        player.x,
        player.y,
        Material,
        "Hardwood Plank",
        1,
        &mut items,
    );
    spawn_item_at(
        player.x,
        player.y,
        Ingredient,
        "Bottle of Water",
        1,
        &mut items,
    );

    let items_on_ground = get_items_at(player.x, player.y, &items);
    let target_item = items_on_ground[2].clone();
    let target_item_id = target_item.id;

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
        &Command::TakeItem(3),
        Some(&update_tx),
        Some(command_tx.clone()),
    );

    assert!(activity.is_none());

    assert_item_removed(target_item_id, &mut update_rx);
    assert_inventory_updated(&mut vec![target_item], &mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}
