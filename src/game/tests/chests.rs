use super::*;

#[test]
fn opening_a_chest_provides_client_with_external_inventory() {
    let (
        mut player,
        mut map,
        mut obstacles,
        mut characters,
        mut item_class_specifiers,
        mut items,
        mut facilities,
        mut inventories,
        mut timer,
        update_tx,
        update_rx,
        mut game_state,
    ) = initialize_game_system();

    game_state.teleport_player(8, 7, &mut player, &mut obstacles, &inventories, None, None);

    game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut timer,
        None,
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );
    let update = update_rx.try_recv().expect("did not receive update");

    let exp_inventory_id = facilities.at(7, 7).unwrap().id;

    let exp_inventory = inventories.get(&exp_inventory_id).unwrap().to_vec();

    match update {
        ExternalInventoryOpened(inventory, inventory_id) if inventory_id == exp_inventory_id => {
            let mut sorted_inventory = inventory.clone();
            sorted_inventory.sort();

            let mut sorted_exp_inventory = exp_inventory;
            sorted_exp_inventory.sort();

            // assert given inventory is the appropriate external inventory
            assert_eq!(sorted_inventory, sorted_exp_inventory);

            // assert player's external inventory is also appropriately set
            let mut sorted_player_external_inventory = player.external_inventory.unwrap();
            sorted_player_external_inventory.sort();

            assert_eq!(sorted_player_external_inventory, sorted_exp_inventory);
        }
        _ => {
            panic!("unexpected update found: {:?}", update);
        }
    }
}

#[test]
fn taking_all_from_a_chest() {
    let (
        mut player,
        mut map,
        mut obstacles,
        mut characters,
        mut item_class_specifiers,
        mut items,
        mut facilities,
        mut inventories,
        mut timer,
        update_tx,
        update_rx,
        mut game_state,
    ) = initialize_game_system();

    game_state.teleport_player(6, 12, &mut player, &mut obstacles, &inventories, None, None);

    game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut timer,
        None,
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        None,
        None,
    );

    let chest_inventory_id = facilities.at(7, 7).unwrap().id;

    let player_inventory = inventories.get(&player.id).unwrap().to_vec();
    let mut chest_inventory = inventories.get(&chest_inventory_id).unwrap().to_vec();

    let mut exp_player_inventory = player_inventory;
    exp_player_inventory.append(&mut chest_inventory);
    exp_player_inventory.sort();

    let activity = game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut timer,
        None,
        &Command::TransferAllItems(chest_inventory_id, 1),
        Some(&update_tx),
        None,
    );

    // make sure that no activity is started
    assert!(activity.is_none());

    // make sure destination inventory is correct
    let update = update_rx.try_recv().expect("did not receive update");

    match update {
        InventoryUpdated(mut inventory) => {
            inventory.sort();
            assert_eq!(inventory, exp_player_inventory)
        }
        _ => panic!("received unexpected update: {:?}", update),
    }

    // make sure source inventory is now empty
    let update = update_rx.try_recv().expect("did not receive update");

    match update {
        ExternalInventoryUpdated(external_inventory) => assert!(external_inventory.is_empty()),
        _ => panic!("received unexpected update: {:?}", update),
    }
}

#[test]
fn stashing_all_to_a_chest() {
    let (
        mut player,
        mut map,
        mut obstacles,
        mut characters,
        mut item_class_specifiers,
        mut items,
        mut facilities,
        mut inventories,
        mut timer,
        update_tx,
        update_rx,
        mut game_state,
    ) = initialize_game_system();

    game_state.teleport_player(6, 12, &mut player, &mut obstacles, &inventories, None, None);

    game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut timer,
        None,
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        None,
        None,
    );

    let chest_inventory_id = facilities.at(7, 7).unwrap().id;

    let player_inventory = inventories.get(&player.id).unwrap();
    let chest_inventory = inventories.get(&chest_inventory_id).unwrap();

    let mut exp_chest_inventory = merge_inventories(chest_inventory, player_inventory).count_all();
    exp_chest_inventory.sort();

    let activity = game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        &mut timer,
        None,
        &Command::TransferAllItems(1, chest_inventory_id),
        Some(&update_tx),
        None,
    );

    // make sure that no activity is started
    assert!(activity.is_none());

    // make sure dest inventory now has all items
    let update = update_rx.try_recv().expect("did not receive update");

    match update {
        ExternalInventoryUpdated(external_inventory) => {
            let mut found_inventory = count_all(external_inventory);
            found_inventory.sort();
            assert!(
                compare_tuple_quantity_arrays(found_inventory.clone(), exp_chest_inventory.clone()),
                "Left:`{:?}`\nRight:`{:?}",
                found_inventory,
                exp_chest_inventory
            );
        }
        _ => panic!("received unexpected update: {:?}", update),
    }

    // make sure soruce inventory is now empty
    let update = update_rx.try_recv().expect("did not receive update");

    match update {
        InventoryUpdated(inventory) => assert!(inventory.is_empty()),
        _ => panic!("received unexpected update: {:?}", update),
    }
}
