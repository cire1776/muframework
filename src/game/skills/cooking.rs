use super::*;
use game::command::facility_commands::fishing_spot_commands::FishType;
use FishType::*;
use ItemClass::*;

lazy_static! {
    static ref COOKING_RECIPES: HashMap<FishType, Recipe> = {
        let mut m = HashMap::new();
        m.insert(Shrimp, Recipe::new("Grilled Shrimp", 1, 65, 9, 3, 1));
        m.insert(Frog, Recipe::new("Fried Frog Legs", 2, 75, 9, 3, 2));
        m.insert(Mackeral, Recipe::new("Cooked Mackeral", 3, 75, 14, 4, 2));
        m.insert(Crab, Recipe::new("Crab Legs", 5, 45, 23, 8, 3));
        m.insert(Catfish, Recipe::new("Grilled Catfish", 7, 75, 8, 5, 2));
        m.insert(Salmon, Recipe::new("Grilled Salmon", 8, 35, 21, 10, 3));
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

pub struct CookingSkill {}

impl CookingSkill {
    pub fn can_produce(product: FishType, _level: u8, inventory: &Inventory) -> bool {
        if !inventory.has_sufficient(Ingredient, product.to_string(), 1) {
            return false;
        }

        if !inventory.has_sufficient(Material, "Softwood Log", 1)
            && !inventory.has_sufficient(Material, "Hardwood Logs", 1)
        {
            return false;
        }

        true
    }
    pub fn consume_from_inventory_for(
        product: FishType,
        player: &Player,
        inventories: &mut InventoryList,
        items: &mut ItemList,
    ) {
        let inventory = inventories
            .get_mut(&player.inventory_id())
            .expect("unable to find inventory");

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
            let delta_level = std::cmp::min(level - recipe.required_level, recipe.learning_period);
            let factor = (100 - recipe.success_rate) / delta_level;
            recipe.success_rate + delta_level * factor
        };

        rng.percentile(success_rate, "cooking_success")
    }

    pub fn produce_results_for(
        product: FishType,
        player: &mut Player,
        rng: &mut Rng,
    ) -> (ItemClass, String) {
        let recipe = &COOKING_RECIPES[&product];

        let level = player.get_level_for(Cooking);

        let success = Self::succeeds(recipe, level, rng);

        if success {
            player.increment_xp(Cooking, recipe.xp_on_success as u64);
            (Food, recipe.success_product.clone())
        } else {
            player.increment_xp(Cooking, recipe.xp_on_failure as u64);
            (Material, format!("Burnt {}", product.to_string()))
        }
    }
}
