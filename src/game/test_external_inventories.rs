use super::*;
use std::sync::mpsc::channel;
use ui::input::InputState;
use ui::UIState;
use ItemClass::*;

#[test]
fn entering_external_inventory_state() {
    let (update_tx, update_rx) = channel();
    let (command_tx, _command_rx) = channel();

    let mut subject = UIState::new(update_rx, command_tx);

    let exp_inventory = vec![Item::new(
        907,
        ItemType::new(ItemClass::Gloves, "a pair of nylon gloves"),
        1,
    )];
    update_tx
        .send(GameUpdate::ExternalInventoryOpened(
            exp_inventory.clone(),
            5,
        ))
        .unwrap();

    subject.perform_tick(None);
    assert_eq!(subject.external_inventory, Some(exp_inventory));
    assert_eq!(subject.input_state, InputState::ExternalInventoryOpen);
}

#[test]
fn leaving_external_inventory_state() {
    let (update_tx, update_rx) = channel();
    let (command_tx, _command_rx) = channel();

    let mut subject = UIState::new(update_rx, command_tx);

    {
        let exp_inventory = vec![Item::new(
            907,
            ItemType::new(ItemClass::Gloves, "a pair of nylon gloves"),
            1,
        )];
        update_tx
            .send(GameUpdate::ExternalInventoryOpened(
                exp_inventory.clone(),
                5,
            ))
            .unwrap();
        subject.perform_tick(None);
    }

    update_tx.send(GameUpdate::ExternalInventoryClosed).unwrap();
    subject.perform_tick(None);

    assert_eq!(subject.external_inventory, None);
    assert_eq!(subject.input_state, InputState::Normal);
}

#[test]
fn escape_ends_external_inventory_mode() {
    let (update_tx, update_rx) = channel();
    let (command_tx, _command_rx) = channel();
    let mut subject = UIState::new(update_rx, command_tx);

    let exp_inventory = vec![Item::new(
        907,
        ItemType::new(ItemClass::Gloves, "a pair of nylon gloves"),
        1,
    )];
    update_tx
        .send(GameUpdate::ExternalInventoryOpened(
            exp_inventory.clone(),
            5,
        ))
        .unwrap();

    subject.perform_tick(None);

    let input = ui::input::Input {
        key: Some(VirtualKeyCode::Escape),
        shift: false,
        control: false,
        alt: false,
    };

    let command = subject.get_command_from_keyboard_input(&input);

    if let Command::CloseExternalInventory = command {
        // Will still be in external inventory mode until update::ExternalInventoryClosed occurs.
        assert_eq!(subject.input_state, InputState::ExternalInventoryOpen);
    } else {
        panic!("command mismatch error: {:?}", command)
    }
}

#[test]
fn opening_external_inventory() {
    let (update_tx, update_rx) = std::sync::mpsc::channel();

    let (
        mut player,
        mut map,
        mut obstacles,
        mut characters,
        mut item_class_specifiers,
        mut items,
        mut facilities,
        mut inventories,
    ) = game::GameState::initialize_game("maps/test.map", None);

    let mut game_state = GameState::new();
    game_state.teleport_player(8, 7, &mut player, &mut obstacles, None, None);

    game_state.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        None,
        &Command::Move(Direction::Left, MoveCommandMode::Use),
        Some(&update_tx),
        None,
    );
    let update = update_rx.try_recv().expect("did not receive update");

    let exp_inventory = vec![
        Item {
            id: 38,
            quantity: 1,
            item_type: ItemType::new(Headwear, "Old leather cap"),
        },
        Item {
            id: 35,
            quantity: 1,
            item_type: ItemType::new(Dagger, "Shiny Dagger"),
        },
        Item {
            id: 36,
            quantity: 1,
            item_type: ItemType::new(BladeWeapon, "Sharp Short Sword"),
        },
        Item {
            id: 37,
            quantity: 1,
            item_type: ItemType::new(Potion, "pink potion"),
        },
    ];

    let exp_inventory_id = facilities.at(7, 7).unwrap().id;

    match update {
        ExternalInventoryOpened(inventory, inventory_id) if inventory_id == exp_inventory_id => {
            // assert given inventory is the appropriate external inventory
            assert_eq!(inventory.clone().sort(), exp_inventory.clone().sort());

            // assert player's external inventory is also appropriately set
            assert_eq!(
                player.external_inventory.map(|i| i.clone().sort()),
                Some(exp_inventory.clone().sort())
            );
        }
        _ => {
            panic!("unexpected update found: {:?}", update);
        }
    }
}

#[test]
#[ignore = "finish alias implementation"]
fn game_state_closing_external_inventory() {
    let mut subject = game::GameState::new();

    let (update_tx, update_rx) = channel();
    let (
        mut player,
        mut map,
        mut obstacles,
        mut characters,
        mut item_class_specifiers,
        mut items,
        mut facilities,
        mut inventories,
    ) = game::GameState::initialize_game("maps/test.map", None);

    subject.game_loop_iteration(
        &mut player,
        &mut map,
        &mut obstacles,
        &mut characters,
        &mut item_class_specifiers,
        &mut items,
        &mut facilities,
        &mut inventories,
        None,
        &Command::CloseExternalInventory,
        Some(&update_tx),
        None,
    );

    let update = update_rx.try_recv().unwrap();

    if let ExternalInventoryClosed = update {
        //passes
    } else {
        panic!("unexpected update found");
    }
}
