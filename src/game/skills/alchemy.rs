use super::*;
use skills::WellType::*;
use ItemClass::*;

pub struct AlchemyFillingSkill {}

impl AlchemyFillingSkill {
    pub fn can_produce(_player: &Player, facility: &Facility) -> bool {
        facility.get_property("fluid") == Oil as i128
    }

    pub fn expiration(player: &Player) -> u32 {
        (30 + player.get_attribute(Attribute::SkillTime(Cooking), 0)) as u32
    }

    pub fn consume_from_inventory_for(
        player: &mut Player,
        inventories: &mut InventoryList,
        items: &mut ItemList,
    ) {
        let inventory = inventories
            .get_mut(&player.inventory_id())
            .expect("unable to find inventory");

        if !inventory.has_sufficient(Material, "Glass Bottle", 1) {
            let item =
                player
                    .mounting_points
                    .unmount(&vec![&MountingPoint::AtReady], inventory, items);
            if let Some(item) = item {
                item.has_been_unequipped(player);
            }
        }

        inventory.consume(Material, "Glass Bottle", 1, items);
    }

    pub fn produce_results_for(
        _player: &mut Player,
        _facility: &mut Facility,
        _rng: &mut Rng,
        _update_tx: Option<&GameUpdateSender>,
    ) -> (ItemClass, String) {
        (Material, "Bottle of Oil".into())
    }
}
