use super::*;
use game::command::facility_commands::lumbermill_commands::LogType;
use ItemClass::*;

pub enum ConstructionSkill {}

impl ConstructionSkill {
    pub fn can_produce(product: LogType, _level: u8, inventory: &Inventory) -> bool {
        let log_type = format!("{} Log", Self::wood(product));
        inventory.has_sufficient(Material, log_type, 1)
    }

    pub fn expiration(product: LogType, player: &Player) -> u32 {
        (match product {
            LogType::Softwood => 40,
            LogType::Hardwood => 60,
        } + player.get_attribute(Attribute::SkillTime(Construction), 0)) as u32
    }

    pub fn consume_from_inventory_for(
        product: LogType,
        player: &mut Player,
        inventories: &mut InventoryList,
        items: &mut ItemList,
    ) {
        let inventory = inventories
            .get_mut(&player.inventory_id())
            .expect("unable to get player's inventory.");

        let wood_type = Self::wood(product);

        if !inventory.has_sufficient(Material, format!("{} Log", wood_type), 1) {
            let item =
                player
                    .mounting_points
                    .unmount(&vec![&MountingPoint::AtReady], inventory, items);
            if let Some(item) = item {
                item.has_been_unequipped(player);
            }
        }
        inventory.consume(Material, format!("{} Log", wood_type), 1, items);
    }

    pub fn produce_results_for(
        product: LogType,
        player: &mut Player,
        rng: &mut Rng,
        update_tx: Option<&GameUpdateSender>,
    ) -> (ItemClass, String) {
        let xp_gain = match product {
            LogType::Softwood => 5,
            LogType::Hardwood => 10,
        };

        player.increment_xp(Construction, xp_gain, rng, update_tx);

        (Material, format!("{} Plank", Self::wood(product)))
    }

    #[inline]
    fn wood(log_type: LogType) -> String {
        use inflector::Inflector;
        log_type.to_string().to_title_case()
    }
}
