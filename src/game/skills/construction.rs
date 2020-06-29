use super::*;
use game::command::facility_commands::lumbermill_commands::LogType;
use ItemClass::*;

pub enum ConstructionSkill {}

impl ConstructionSkill {
    pub fn can_produce(product: LogType, _level: u8, inventory: &Inventory) -> bool {
        let log_type = format!("{} Log", Self::wood(product));
        inventory.has_sufficient(Material, log_type, 1)
    }

    pub fn consume_from_inventory_for(
        product: LogType,
        player: &Player,
        inventories: &mut InventoryList,
        items: &mut ItemList,
    ) {
        let inventory = inventories
            .get_mut(&player.inventory_id())
            .expect("unable to get player's inventory.");

        let wood_type = Self::wood(product);
        inventory.consume(Material, format!("{} Plank", wood_type), 1, items);
    }

    pub fn produce_results_for(
        product: LogType,
        player: &mut Player,
        _rng: &mut Rng,
    ) -> (ItemClass, String) {
        let xp_gain = match product {
            LogType::Softwood => 5,
            LogType::Hardwood => 10,
        };

        player.increment_xp("construction", xp_gain);

        (Material, format!("{} Plank", Self::wood(product)))
    }

    #[inline]
    fn wood(log_type: LogType) -> String {
        use inflector::Inflector;
        log_type.to_string().to_title_case()
    }
}
