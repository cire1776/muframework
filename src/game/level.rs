use super::*;

pub struct Level {}

impl Level {
    pub fn introduce(
        player: &Player,
        map: &mut TileMap,
        obstacles: &mut BlockingMap,
        characters: &CharacterList,
        facilities: &mut FacilityList,
        items: &ItemList,
        inventories: &InventoryList,
        update_tx: Option<&GameUpdateSender>,
    ) {
        Level::introduce_player(&player, inventories, update_tx);
        Level::introduce_other_characters(&characters, obstacles, update_tx);
        Level::introduce_items(&items, update_tx);
        Level::introduce_facilities(facilities, map, obstacles, update_tx);
    }

    pub fn introduce_player(
        player: &Player,
        inventories: &InventoryList,
        update_tx: Option<&GameUpdateSender>,
    ) {
        GameUpdate::send(
            update_tx,
            CharacterEntered {
                id: player.id,
                x: player.x,
                y: player.y,
                character_type: CharacterType::Player,
            },
        );

        for (skill, xp_value) in player.skills_with_xp() {
            GameUpdate::send(
                update_tx,
                PlayerXPUpdated(player.id, skill.to_string(), *xp_value),
            )
        }

        for (skill, _) in player.skills() {
            GameUpdate::send(
                update_tx,
                PlayerSkillUpdated(player.id, skill.to_string(), player.get_level_for(skill)),
            )
        }

        let inventory = inventories.get(&1).unwrap();
        GameUpdate::send(update_tx, InventoryUpdated(inventory.to_vec()));
    }

    pub fn introduce_other_characters(
        characters: &CharacterList,
        obstacles: &mut BlockingMap,
        update_tx: Option<&GameUpdateSender>,
    ) {
        for character in characters.iter() {
            GameUpdate::send(
                update_tx,
                CharacterEntered {
                    id: character.id,
                    x: character.x,
                    y: character.y,
                    character_type: character.character_type,
                },
            );
            obstacles.block_at(character.x, character.y);
        }
    }

    pub fn introduce_items(items: &ItemList, update_tx: Option<&GameUpdateSender>) {
        for (index, item) in items.iter() {
            match item {
                ItemState::Bundle(item, x, y) => {
                    GameUpdate::send(
                        update_tx,
                        ItemAdded {
                            id: *index,
                            x: *x,
                            y: *y,
                            class: item.class(),
                            description: item.description(),
                        },
                    );
                }
                _ => {} // only introduce bundles
            }
        }
    }

    pub fn introduce_facilities(
        facilities: &mut FacilityList,
        map: &mut TileMap,
        obstacles: &mut BlockingMap,
        update_tx: Option<&GameUpdateSender>,
    ) {
        for (index, facility) in facilities.iter_mut() {
            GameUpdate::send(
                update_tx,
                FacilityAdded {
                    id: *index,
                    x: facility.x,
                    y: facility.y,
                    class: facility.class.clone(),
                    description: facility.description.clone(),
                    variant: facility.variant(),
                },
            );
            obstacles.block_at(facility.x, facility.y);

            {
                // will eventually need to move to facility.
                facility.background_tile = map.at(facility.x, facility.y);
                map.set_tile_at(facility.x, facility.y, tile_map::Tile::Facility(*index));
            }
        }
    }
}
