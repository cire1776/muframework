use super::*;
#[test]
fn unequip_command_removes_endorsements() {
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
    ) = initialize_game_system_with_player_at(10, 8);

    player.endorse_with(":can_fish");

    equip_player_with_spawned_item(
        Tool,
        "Simple Fishing Rod",
        &mut player,
        &mut inventories,
        &mut items,
    );

    give_player_spawned_items(
        Tool,
        "Simple Fishing Net",
        1,
        &mut player,
        &mut inventories,
        &mut items,
    );

    let inventory = inventories
        .get(&player.id)
        .expect("unable to get inventory.");

    let item = inventory.find(Tool, "Simple Fishing Net").unwrap();

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
        None,
        &Command::EquipItem(item.id),
        Some(&update_tx),
        None,
    );

    assert!(player.is_endorsed_with(":can_net_fish"));
    assert!(!player.is_endorsed_with(":can_fish"));

    assert_is_equipment_updated(vec![item], &mut update_rx);
    assert_is_inventory_updated(&mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);
}

#[test]
fn unequip_removes_endorsement_components() {
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
    ) = initialize_game_system_with_player_at(10, 8);

    player.endorse_component_with(":wants_to_mill", "Softwood");

    let item_id = equip_player_with_spawned_item(
        Material,
        "Softwood Log",
        &mut player,
        &mut inventories,
        &mut items,
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
        None,
        &Command::UnequipItem(item_id),
        Some(&update_tx),
        None,
    );

    assert_is_equipment_updated(vec![], &mut update_rx);
    assert_is_inventory_updated(&mut update_rx);
    assert_updates_are_empty(&mut update_rx);
    assert_commands_are_empty(&mut command_rx);

    assert_eq!(player.get_endorsement_component(":wants_to_mill"), None)
}
