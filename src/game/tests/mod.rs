use super::*;
use std::sync::mpsc::*;
// use ui::input::InputState;
// use ui::UIState;
// use ItemClass::*;

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
    extern_timer::Timer,
    Sender<GameUpdate>,
    Receiver<GameUpdate>,
    GameState,
) {
    let (update_tx, update_rx) = channel();

    let (
        player,
        map,
        obstacles,
        characters,
        item_class_specifiers,
        items,
        facilities,
        inventories,
        timer,
    ) = game::GameState::initialize_game("maps/test.map", None);

    let game_state = GameState::new();

    (
        player,
        map,
        obstacles,
        characters,
        item_class_specifiers,
        items,
        facilities,
        inventories,
        timer,
        update_tx,
        update_rx,
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

pub fn compare_tuple_quantity_arrays(
    array1: Vec<(ItemType, u16)>,
    array2: Vec<(ItemType, u16)>,
) -> bool {
    array1
        .iter()
        .zip(&array2)
        .all(|(a, b)| a.0 == b.0 && a.1 == b.1)
}

#[cfg(test)]
mod chests;
