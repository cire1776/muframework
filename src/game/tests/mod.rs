use super::*;
use std::option::Option::None;
use std::sync::mpsc::*;
use ui::pane::PaneTitle::*;
use ItemClass::*;

// use ui::input::InputState;
// use ui::UIState;

#[cfg(test)]
pub fn initialize_game_system() -> (
    Player,
    TileMap,
    BlockingMap,
    CharacterList,
    ItemClassSpecifierList,
    ItemList,
    FacilityList,
    InventoryList,
    Rng,
    Timer,
    Sender<GameUpdate>,
    Receiver<GameUpdate>,
    CommandSender,
    Receiver<Command>,
    GameState,
) {
    let (update_tx, update_rx) = channel();
    let (command_tx, command_rx) = channel();

    let (
        player,
        map,
        obstacles,
        characters,
        item_class_specifiers,
        items,
        facilities,
        inventories,
        mut timer,
    ) = game::GameState::initialize_game("maps/test.map", None, Some(command_tx.clone()));

    let game_state = GameState::new();

    let mut rng = Rng::new();
    rng.set_test_mode();

    timer.set_test_mode();

    (
        player,
        map,
        obstacles,
        characters,
        item_class_specifiers,
        items,
        facilities,
        inventories,
        rng,
        timer,
        update_tx,
        update_rx,
        command_tx,
        command_rx,
        game_state,
    )
}

pub fn initialize_game_system_with_player_at(
    x: i32,
    y: i32,
) -> (
    Player,
    TileMap,
    BlockingMap,
    CharacterList,
    ItemClassSpecifierList,
    ItemList,
    FacilityList,
    InventoryList,
    Rng,
    Timer,
    Sender<GameUpdate>,
    Receiver<GameUpdate>,
    CommandSender,
    Receiver<Command>,
    GameState,
) {
    let (
        mut player,
        map,
        mut obstacles,
        characters,
        item_class_specifiers,
        items,
        facilities,
        mut inventories,
        rng,
        timer,
        update_tx,
        update_rx,
        command_tx,
        command_rx,
        mut game_state,
    ) = initialize_game_system();

    game_state.teleport_player(
        x,
        y,
        &mut player,
        &mut obstacles,
        &mut inventories,
        None,
        None,
    );
    (
        player,
        map,
        obstacles,
        characters,
        item_class_specifiers,
        items,
        facilities,
        inventories,
        rng,
        timer,
        update_tx,
        update_rx,
        command_tx,
        command_rx,
        game_state,
    )
}

#[cfg(test)]
pub fn merge_inventories(inventory1: &Inventory, inventory2: &Inventory) -> Inventory {
    let result = inventory1.clone();
    let mut items = ItemList::new(None);

    let result = inventory2
        .items
        .iter()
        .fold(result.clone(), |mut accum, (_, i)| {
            accum.accept_stack_unmut(i, &mut items);
            accum
        });

    result
}

pub fn count_all(items: Vec<Item>) -> Vec<(ItemType, u16)> {
    let counts: HashMap<ItemType, u16> = HashMap::new();

    let counts = items.iter().fold(counts, |mut accum, i| {
        let lineitem = accum.get_mut(&i.item_type);
        if let Some(quantity) = lineitem {
            *quantity += i.quantity as u16;
            accum
        } else {
            accum.insert(i.item_type.clone(), i.quantity as u16);
            accum
        }
    });

    let result = counts.iter().map(|i| (i.0.clone(), *i.1)).collect();
    result
}

pub fn get_facility_id_at(x: i32, y: i32, map: &TileMap) -> u64 {
    match map.at(x, y) {
        tile_map::Tile::Facility(id) => id,
        _ => panic!("facility not found"),
    }
}

pub fn compare_tuple_quantity_arrays(
    array1: Vec<(ItemType, u16)>,
    array2: Vec<(ItemType, u16)>,
) -> bool {
    array1
        .iter()
        .zip(&array2)
        .all(|(a, b)| a.0 == b.0 && a.1 == b.1)
}

pub fn give_player_level<S: ToString>(skill: S, level: u8, player: &mut Player) {
    let skill = skill.to_string();
    player.remove_buff(BuffTag::Level(skill.clone()));
    player.add_buff(
        Attribute::SkillLevel(skill.clone()),
        (level as i8, 0, BuffTag::Level(skill)),
    );
}

#[allow(dead_code)]
pub fn equip_player_with_spawned_item<S: ToString>(
    class: ItemClass,
    description: S,
    player: &mut Player,
    inventories: &mut InventoryList,
    items: &mut ItemList,
) {
    let inventory = inventories
        .get_mut(&player.id)
        .expect("unable to get inventory");

    let item = inventory.spawn_stack(class, description, 1, items);

    let item_class_specifiers = ItemClassSpecifier::initialize();

    player
        .mounting_points
        .mount(&item, &item_class_specifiers, inventory, items);
}

#[cfg(test)]
pub fn give_player_spawned_items<S: ToString>(
    class: ItemClass,
    description: S,
    quantity: u8,
    player: &mut Player,
    inventories: &mut InventoryList,
    items: &mut ItemList,
) {
    let inventory = inventories
        .get_mut(&player.id)
        .expect("unable to get inventory");

    inventory.spawn_stack(class, description, quantity, items);
}

pub fn assert_activity_started(
    duration: u32,
    exp_title: ui::pane::PaneTitle,
    update_rx: &mut Receiver<GameUpdate>,
) {
    let update = update_rx.try_recv().unwrap();

    match update {
        GameUpdate::ActivityStarted(millis, title) => {
            assert_eq!(millis, duration);
            assert_eq!(title, exp_title);
        }
        _ => panic!("unexpected update"),
    }
}

pub fn assert_activity_expired(update_rx: &mut Receiver<GameUpdate>) {
    let update = update_rx.try_recv().unwrap();

    match update {
        GameUpdate::ActivityExpired() => {}
        _ => panic!("unexpected update"),
    }
}

pub fn assert_is_equipment_updated(exp_items: Vec<Item>, update_rx: &mut Receiver<GameUpdate>) {
    let update = update_rx.try_recv();

    match update {
        Ok(EquipmentUpdated(items)) if items == exp_items => {}
        _ => panic!("unexpected update: {:?}", update),
    }
}
pub fn assert_is_display_options<S: ToString>(
    exp_options: Vec<S>,
    exp_continuation: ActionContinuation,
    update_rx: &mut Receiver<GameUpdate>,
) {
    let update = update_rx.try_recv();
    let exp_options: Vec<String> = exp_options.iter().map(|s| s.to_string()).collect();
    match update {
        Ok(DisplayOptions(options, action_continuation, _facility_id))
            if options == exp_options && exp_continuation == action_continuation => {}
        _ => panic!("unexpected update: {:?}", update),
    }
}

pub fn assert_is_inventory_updated(update_rx: &mut Receiver<GameUpdate>) {
    let update = update_rx.try_recv();

    match update {
        Ok(InventoryUpdated(_)) => {}
        _ => panic!("unexpected update: {:?}", update),
    }
}

pub fn assert_updates_are_empty(update_rx: &mut Receiver<GameUpdate>) {
    let update = update_rx.try_recv();

    match update {
        Ok(update) => panic!("updates not empty:{:?}", update),
        Err(_) => {}
    }
}

pub fn assert_commands_are_empty(command_rx: &mut Receiver<Command>) {
    let command = command_rx.try_recv();

    match command {
        Ok(command) => panic!("commands not empty:{:?}", command),
        Err(_) => {}
    }
}

pub fn assert_is_refresh_inventory(command_rx: &mut Receiver<Command>) {
    let command = command_rx.try_recv();

    match command {
        Ok(Command::RefreshInventory) => {}
        Ok(command) => panic!("unexpected command found: {:?}", command),
        Err(_) => panic!("command not found."),
    }
}

pub fn assert_is_spawning_item<S: ToString>(
    inventory_id: u64,
    class: ItemClass,
    description: S,
    command_rx: &mut Receiver<Command>,
) {
    let command = command_rx.try_recv().unwrap();

    match command {
        Command::SpawnItem(id, c, d)
            if id == inventory_id && c == class && d == description.to_string() => {}
        _ => panic!("unexpected command: {:?}", command),
    }
}

pub fn assert_is_destroy_facility(command_rx: &mut Receiver<Command>) -> Command {
    let command = command_rx.try_recv().unwrap();

    match command {
        Command::DestroyFacility(_id) => Command::None,
        _ => panic!("unexpected command: {:?}", command),
    }
}

pub fn assert_is_activity_abort(command_rx: &mut Receiver<Command>) -> Command {
    let command = command_rx.try_recv();

    match command {
        Ok(Command::ActivityAbort) => Command::None,
        Err(TryRecvError::Empty) => panic!("No command received. Expected ActivityAbort"),
        _ => panic!("unexpected command: {:?}", command),
    }
}

#[allow(dead_code)]
pub fn assert_player_has<S: ToString>(
    class: ItemClass,
    description: S,
    quantity: u16,
    player: &Player,
    inventories: &InventoryList,
) {
    let item_type = ItemType::new(class, description);
    let inventory = inventories
        .get(&player.id)
        .expect("unable to find inventory");

    let items = inventory.count_all();

    assert!(
        items.iter().any(|i| i.0 == item_type && i.1 == quantity),
        "{:?}",
        items
    );
}

#[cfg(test)]
mod chests;

#[cfg(test)]
mod trees;

#[cfg(test)]
mod veins;

#[cfg(test)]
mod smeltery;

#[cfg(test)]
mod wells;

#[cfg(test)]
mod equipment;
