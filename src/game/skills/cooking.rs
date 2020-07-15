use super::*;
use game::command::facility_commands::fishing_spot_commands::FishType;
use FishType::*;
use ItemClass::*;
use WellType::*;

lazy_static! {
    static ref COOKING_RECIPES: HashMap<FishType, Recipe> = {
        let mut m = HashMap::new();
        m.insert(Shrimp, Recipe::new("Grilled Shrimp", 1, 65, 9, 3, 1));
        m.insert(Frog, Recipe::new("Fried Frog Leg", 2, 75, 9, 3, 2));
        m.insert(Mackeral, Recipe::new("Cooked Mackeral", 3, 75, 14, 4, 2));
        m.insert(Crab, Recipe::new("Crab Leg", 5, 45, 23, 8, 3));
        m.insert(Catfish, Recipe::new("Grilled Catfish", 7, 75, 8, 5, 2));
        m.insert(Salmon, Recipe::new("Grilled Salmon", 8, 35, 21, 10, 3));
        m.insert(Bass, Recipe::new("Cooked Bass", 11, 40, 20, 11, 4));
        m.insert(Flounder, Recipe::new("Cooked Flounder", 15, 45, 30, 12, 4));
        m.insert(Haddock, Recipe::new("Cooked Haddock", 17, 50, 13, 13, 5));
        m.insert(Swordfish, Recipe::new("Swordfish Steak", 19, 80, 16, 12, 5));
        m.insert(Eel, Recipe::new("Cooked Eel", 21, 20, 5, 20, 5));
        m.insert(Sardine, Recipe::new("Canned Sardines", 23, 100, 0, 10, 0));
        m.insert(SandbarShark, Recipe::new("Shark Steak", 25, 20, 20, 15, 6));
        m.insert(Pike, Recipe::new("Cooked Pike", 27, 10, 13, 20, 7));
        m.insert(Lobster, Recipe::new("Steamed Lobster", 29, 90, 5, 17, 5));
        m.insert(Tuna, Recipe::new("Tuna Steak", 31, 30, 11, 22, 8));
        m.insert(
            StripedMarlin,
            Recipe::new("Grilled Marlin", 33, 50, 6, 20, 8),
        );
        m.insert(Herring, Recipe::new("Pickled Herring", 35, 40, 5, 22, 9));
        m.insert(Trout, Recipe::new("Grilled Trout", 37, 50, 5, 22, 9));
        m.insert(Snapper, Recipe::new("Grilled Snapper", 39, 20, 6, 25, 10));
        m.insert(Cod, Recipe::new("Fishsticks", 41, 80, 4, 20, 10));
        m.insert(Sturgeon, Recipe::new("Grilled Sturgeon", 43, 40, 2, 22, 10));
        m.insert(
            GiantCatfish,
            Recipe::new("Breaded Catfish", 45, 80, 1, 25, 10),
        );
        m.insert(Grouper, Recipe::new("Cooked Grouper", 45, 90, 1, 25, 10));
        m.insert(
            BlackSeaBass,
            Recipe::new("Grilled Black Sea Bass", 45, 80, 1, 26, 10),
        );

        m
    };
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Recipe {
    success_product: String,
    required_level: u8,
    success_rate: u8,
    learning_period: u8,
    xp_on_success: u8,
    xp_on_failure: u8,
}

impl Recipe {
    pub fn new<S: ToString>(
        success_product: S,
        required_level: u8,
        success_rate: u8,
        learning_period: u8,
        xp_on_success: u8,
        xp_on_failure: u8,
    ) -> Self {
        Self {
            success_product: success_product.to_string(),
            required_level,
            success_rate,
            learning_period,
            xp_on_success,
            xp_on_failure,
        }
    }
}

pub struct CookingFishSkill {}

impl CookingFishSkill {
    pub fn can_produce(
        product: FishType,
        player: &Player,
        inventory: &Inventory,
        items: &ItemList,
    ) -> bool {
        if !inventory.has_sufficient(Ingredient, product.to_string(), 1) {
            let mounted_item_id = player.mounting_points.at(&MountingPoint::AtReady);

            if let Some(mounted_item_id) = mounted_item_id {
                let mounted_item = items
                    .get_as_item(mounted_item_id)
                    .expect("can't find item.");

                return mounted_item.is_of_type(Ingredient, product.to_string());
            } else {
                return false;
            }
        }

        if !inventory.has_sufficient(Material, "Softwood Log", 1)
            && !inventory.has_sufficient(Material, "Hardwood Logs", 1)
        {
            return false;
        }

        true
    }

    pub fn expiration(_product: FishType, player: &Player) -> u32 {
        (60 + player.get_attribute(Attribute::SkillTime(Cooking), 0)) as u32
    }

    pub fn consume_from_inventory_for(
        product: FishType,
        player: &mut Player,
        inventories: &mut InventoryList,
        items: &mut ItemList,
    ) {
        let inventory = inventories
            .get_mut(&player.inventory_id())
            .expect("unable to find inventory");

        if !inventory.has_sufficient(Ingredient, product.to_string(), 1) {
            let item =
                player
                    .mounting_points
                    .unmount(&vec![&MountingPoint::AtReady], inventory, items);
            if let Some(item) = item {
                item.has_been_unequipped(player);
            }
        }

        inventory.consume(Ingredient, product.to_string(), 1, items);

        if inventory.has_sufficient(Material, "Softwood Log", 1) {
            inventory.consume(Material, "Softwood Log", 1, items);
        } else if inventory.has_sufficient(Material, "Hardwood Log", 1) {
            inventory.consume(Material, "Hardwood Log", 1, items);
        }
    }

    fn succeeds(recipe: &Recipe, level: u8, rng: &mut Rng) -> bool {
        let success_rate = if level < recipe.required_level {
            0
        } else {
            let delta_level = level - recipe.required_level;

            if delta_level >= recipe.learning_period {
                100
            } else {
                let factor = (100 - recipe.success_rate) / recipe.learning_period;
                recipe.success_rate + delta_level * factor
            }
        };

        rng.percentile(success_rate, "cooking_success")
    }

    pub fn produce_results_for(
        product: FishType,
        player: &mut Player,
        rng: &mut Rng,
        update_tx: Option<&GameUpdateSender>,
    ) -> (ItemClass, String) {
        let recipe = &COOKING_RECIPES[&product];

        let level = player.get_level_for(Cooking);

        let success = Self::succeeds(recipe, level, rng);

        if success {
            player.increment_xp(Cooking, recipe.xp_on_success as u64, rng, update_tx);
            (Food, recipe.success_product.clone())
        } else {
            player.increment_xp(Cooking, recipe.xp_on_failure as u64, rng, update_tx);
            (Material, format!("Burnt {}", product.to_string()))
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum WellType {
    Dry = 0,
    Water,
    Oil,
    Bedrock = 255,
}

impl ToString for WellType {
    fn to_string(&self) -> String {
        match self {
            Dry => "Dry",
            Water => "Water",
            Oil => "Oil",
            Bedrock => "Bedrock",
        }
        .to_string()
    }
}

impl WellType {
    pub fn from(value: i128) -> WellType {
        match value {
            0 => Dry,
            1 => Water,
            2 => Oil,
            255 => Bedrock,
            _ => panic!("unknown well type"),
        }
    }
}

pub struct CookingFillingSkill {}

impl CookingFillingSkill {
    pub fn can_produce(_player: &Player, facility: &Facility) -> bool {
        let fluid = facility.get_property("fluid");
        fluid == Water as i128
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
        (Ingredient, "Bottle of Water".into())
    }
}
