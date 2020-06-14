use super::*;

pub struct Level {}

impl Level {
    pub fn new(
        game_state: &mut game::GameState,
        update_tx: Option<&std::sync::mpsc::Sender<GameUpdate>>,
    ) {
        Self::introduce_player(game_state, update_tx);
        Self::introduce_other_characters(game_state, update_tx);
        Self::introduce_items(game_state, update_tx);
        Self::introduce_facilities(game_state, update_tx);

        GameUpdate::send(update_tx, SetBackground(game_state.game_data.map.clone()));
    }

    fn introduce_player(game_state: &game::GameState, update_tx: Option<&GameUpdateSender>) {
        GameUpdate::send(
            update_tx,
            CharacterEntered {
                id: game_state.game_data.player.id,
                x: game_state.game_data.player.x,
                y: game_state.game_data.player.y,
                character_type: CharacterType::Player,
            },
        );

        let inventory = game_state.game_data.inventories.get(&1).unwrap();
        GameUpdate::send(update_tx, InventoryUpdated(inventory.to_vec()));
    }

    fn introduce_other_characters(
        game_state: &mut game::GameState,
        update_tx: Option<&GameUpdateSender>,
    ) {
        for character in game_state.game_data.characters.iter() {
            GameUpdate::send(
                update_tx,
                CharacterEntered {
                    id: character.id,
                    x: character.x,
                    y: character.y,
                    character_type: character.character_type,
                },
            );
            game_state
                .game_data
                .obstacles
                .block_at(character.x, character.y);
        }
    }

    fn introduce_items(game_state: &game::GameState, update_tx: Option<&GameUpdateSender>) {
        for (index, item) in game_state.game_data.items.iter() {
            match item {
                ItemState::Bundle(item, x, y) => {
                    GameUpdate::send(
                        update_tx,
                        ItemAdded {
                            id: *index,
                            x: *x,
                            y: *y,
                            class: item.class,
                            description: item.description.clone(),
                        },
                    );
                }
                _ => {} // only introduce bundles
            }
        }
    }

    fn introduce_facilities(
        game_state: &mut game::GameState,
        update_tx: Option<&GameUpdateSender>,
    ) {
        for (index, facility) in game_state.game_data.facilities.iter() {
            GameUpdate::send(
                update_tx,
                FacilityAdded {
                    id: *index,
                    x: facility.x,
                    y: facility.y,
                    class: facility.class.clone(),
                    description: facility.description.clone(),
                },
            );
            game_state
                .game_data
                .obstacles
                .block_at(facility.x, facility.y);
            game_state.game_data.map.set_tile_at(
                facility.x,
                facility.y,
                tile_map::Tile::Facility(*index),
            );
        }
    }
}
