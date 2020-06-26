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
    Sender<GameUpdate>,
    Receiver<GameUpdate>,
    GameState,
) {
    let (update_tx, update_rx) = channel();

    let (player, map, obstacles, characters, item_class_specifiers, items, facilities, inventories) =
        game::GameState::initialize_game("maps/test.map", None);

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
        update_tx,
        update_rx,
        game_state,
    )
}

#[cfg(test)]
mod chests;
