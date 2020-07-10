use super::*;
use SmeltingType::*;

lazy_static! {
    static ref SMELTING_PRODUCTS: HashMap<SmeltingType, SmeltingProduct> = {
        let mut m = HashMap::new();
        m.insert(Tin, SmeltingProduct::new(1, 5));
        m.insert(Copper, SmeltingProduct::new(2, 6));
        m.insert(Bronze, SmeltingProduct::new(4, 7));
        m.insert(Lead, SmeltingProduct::new(6, 8));
        m.insert(Mercury, SmeltingProduct::new(9, 10));
        m.insert(Iron, SmeltingProduct::new(12, 15));
        m.insert(Tungsten, SmeltingProduct::new(12, 15));
        m.insert(Cobalt, SmeltingProduct::new(15, 20));
        m.insert(Nickel, SmeltingProduct::new(18, 25));
        m.insert(Steel, SmeltingProduct::new(21, 30));
        m.insert(Gold, SmeltingProduct::new(24, 35));
        m.insert(Aluminum, SmeltingProduct::new(27, 40));
        m.insert(Silver, SmeltingProduct::new(30, 45));
        m.insert(Zinc, SmeltingProduct::new(33, 50));
        m.insert(Platinum, SmeltingProduct::new(36, 55));
        m.insert(StainlessSteel, SmeltingProduct::new(39, 60));
        m.insert(Stellite, SmeltingProduct::new(40, 65));
        m.insert(Titanium, SmeltingProduct::new(43, 70));
        m.insert(Mythral, SmeltingProduct::new(45, 75));
        m
    };
}

#[derive(Debug, Copy, Clone)]
pub struct SmeltingProduct {
    smelting_level: u8,
    smelting_xp: u8,
}

impl SmeltingProduct {
    pub fn new(smelting_level: u8, smelting_xp: u8) -> Self {
        Self {
            smelting_level,
            smelting_xp,
        }
    }
}
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum SmeltingType {
    Tin,
    Copper,
    Bronze,
    Lead,
    Mercury,
    Iron,
    Tungsten,
    Cobalt,
    Nickel,
    Steel,
    Gold,
    Aluminum,
    Silver,
    Zinc,
    Platinum,
    StainlessSteel,
    Stellite,
    Titanium,
    Mythral,
}

impl ToString for SmeltingType {
    fn to_string(&self) -> String {
        match self {
            Tin => "Tin",
            Copper => "Copper",
            Bronze => "Bronze",
            Lead => "Lead",
            Mercury => "Mercury",
            Iron => "Iron",
            Tungsten => "Tungsten",
            Cobalt => "Cobalt",
            Nickel => "Nickel",
            Steel => "Steel",
            Gold => "Gold",
            Aluminum => "Aluminum",
            Silver => "Silver",
            Zinc => "Zinc",
            Platinum => "Platinum",
            StainlessSteel => "Stainless Steel",
            Stellite => "Stellite",
            Titanium => "Titanium",
            Mythral => "Mythral",
        }
        .to_string()
    }
}
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum SmeltingSkill {}

impl SmeltingSkill {
    pub fn products_for_player_level(player: &Player) -> Vec<SmeltingType> {
        let level = std::cmp::max(1, player.get_level_for(Smelting));

        let mut sorted_product_list = SMELTING_PRODUCTS
            .iter()
            .collect::<Vec<(&SmeltingType, &SmeltingProduct)>>();

        sorted_product_list.sort_by(|e, other| e.0.cmp(other.0));

        let products = sorted_product_list
            .iter()
            .take_while(|(_, p)| p.smelting_level <= level)
            .map(|(p, _)| **p)
            .collect();

        products
    }

    pub fn can_produce(product: SmeltingType, inventory: &Inventory) -> bool {
        match product {
            Tin | Copper | Lead | Iron | Tungsten | Cobalt | Nickel => {
                if !inventory.has_sufficient(
                    ItemClass::Ore,
                    format!("{} Ore", product.to_string()),
                    4,
                ) {
                    return false;
                }

                if !inventory.has_sufficient(ItemClass::Material, "Softwood Log", 1)
                    && !inventory.has_sufficient(ItemClass::Material, "Hardwood Log", 1)
                    && !inventory.has_sufficient(ItemClass::Material, "Coal", 1)
                {
                    return false;
                }
                true
            }
            Bronze => {
                if !inventory.has_sufficient(ItemClass::Ore, "Tin Ore", 2) {
                    return false;
                }

                if !inventory.has_sufficient(ItemClass::Ore, "Copper Ore", 2) {
                    return false;
                }

                if !inventory.has_sufficient(ItemClass::Material, "Softwood Log", 1)
                    && !inventory.has_sufficient(ItemClass::Material, "Hardwood Log", 1)
                    && !inventory.has_sufficient(ItemClass::Material, "Coal", 1)
                {
                    return false;
                }
                true
            }
            Mercury => {
                if !inventory.has_sufficient(ItemClass::Ore, "Cinnabar", 4) {
                    return false;
                }

                if !inventory.has_sufficient(ItemClass::Material, "Softwood Log", 1)
                    && !inventory.has_sufficient(ItemClass::Material, "Hardwood Log", 1)
                    && !inventory.has_sufficient(ItemClass::Material, "Coal", 1)
                {
                    return false;
                }
                true
            }
            Gold | Silver | Platinum => {
                if !inventory.has_sufficient(
                    ItemClass::Ore,
                    format!("{} Ore", product.to_string()),
                    3,
                ) {
                    return false;
                }

                if !inventory.has_sufficient(ItemClass::Ore, "Mercury", 1) {
                    return false;
                }

                if !inventory.has_sufficient(ItemClass::Material, "Coal", 1) {
                    return false;
                }
                true
            }
            Aluminum | Zinc | Titanium | Mythral => {
                if !inventory.has_sufficient(
                    ItemClass::Ore,
                    format!("{} Ore", product.to_string()),
                    4,
                ) {
                    return false;
                }

                if !inventory.has_sufficient(ItemClass::Ore, "Coal", 1) {
                    return false;
                }
                true
            }
            Steel => {
                if !inventory.has_sufficient(ItemClass::Ore, "Iron Ore", 3) {
                    return false;
                }

                if !inventory.has_sufficient(ItemClass::Ore, "Coal", 2) {
                    return false;
                }
                true
            }
            StainlessSteel => {
                if !inventory.has_sufficient(ItemClass::Ore, "Iron Ore", 3) {
                    return false;
                }

                if !inventory.has_sufficient(ItemClass::Ore, "Zinc Bar", 1) {
                    return false;
                }

                if !inventory.has_sufficient(ItemClass::Ore, "Coal", 1) {
                    return false;
                }
                true
            }
            Stellite => {
                if !inventory.has_sufficient(ItemClass::Ore, "Iron Ore", 3) {
                    return false;
                }

                if !inventory.has_sufficient(ItemClass::Ore, "Tungsten Bar", 1) {
                    return false;
                }

                if !inventory.has_sufficient(ItemClass::Ore, "Coal", 1) {
                    return false;
                }
                true
            }
        }
    }

    pub fn expiration(_product: SmeltingType, player: &Player) -> u32 {
        (60 + player.get_attribute(Attribute::SkillTime(Smelting), 0)) as u32
    }

    pub fn consume_from_inventory_for(
        product: SmeltingType,
        inventory: &mut Inventory,
        items: &mut ItemList,
    ) {
        match product {
            Tin | Copper | Lead | Iron | Tungsten | Cobalt | Nickel => {
                inventory.consume(
                    ItemClass::Ore,
                    format!("{} Ore", product.to_string()),
                    4,
                    items,
                );

                if !inventory.has_sufficient(ItemClass::Material, "Softwood Log", 1) {
                    inventory.consume(ItemClass::Material, "Softwood Log", 1, items);
                } else if inventory.has_sufficient(ItemClass::Material, "Hardwood Log", 1) {
                    inventory.consume(ItemClass::Material, "Hardwood Log", 1, items);
                } else if inventory.has_sufficient(ItemClass::Material, "Coal", 1) {
                    inventory.consume(ItemClass::Ore, "Coal", 1, items);
                } else {
                    panic!("didn't have fuel")
                }
            }
            Bronze => {
                inventory.consume(ItemClass::Ore, "Tin Ore", 2, items);
                inventory.consume(ItemClass::Ore, "Copper Ore", 2, items);

                if !inventory.has_sufficient(ItemClass::Material, "Softwood Log", 1) {
                    inventory.consume(ItemClass::Material, "Softwood Log", 1, items);
                } else if inventory.has_sufficient(ItemClass::Material, "Hardwood Log", 1) {
                    inventory.consume(ItemClass::Material, "Hardwood Log", 1, items);
                } else if inventory.has_sufficient(ItemClass::Material, "Coal", 1) {
                    inventory.consume(ItemClass::Ore, "Coal", 1, items);
                } else {
                    panic!("didn't have fuel")
                }
            }
            Mercury => {
                inventory.consume(ItemClass::Ore, "Cinnabar", 4, items);

                if !inventory.has_sufficient(ItemClass::Material, "Softwood Log", 1) {
                    inventory.consume(ItemClass::Material, "Softwood Log", 1, items);
                } else if inventory.has_sufficient(ItemClass::Material, "Hardwood Log", 1) {
                    inventory.consume(ItemClass::Material, "Hardwood Log", 1, items);
                } else if inventory.has_sufficient(ItemClass::Material, "Coal", 1) {
                    inventory.consume(ItemClass::Ore, "Coal", 1, items);
                } else {
                    panic!("didn't have fuel")
                }
            }
            Gold | Silver | Platinum => {
                inventory.consume(
                    ItemClass::Ore,
                    format!("{} Ore", product.to_string()),
                    3,
                    items,
                );
                inventory.consume(ItemClass::Ore, "Mercury", 1, items);
                inventory.consume(ItemClass::Material, "Coal", 1, items)
            }
            Aluminum | Zinc | Titanium | Mythral => {
                inventory.consume(
                    ItemClass::Ore,
                    format!("{} Ore", product.to_string()),
                    4,
                    items,
                );
                inventory.consume(ItemClass::Ore, "Coal", 1, items)
            }
            Steel => {
                inventory.consume(ItemClass::Ore, "Iron Ore", 3, items);
                inventory.consume(ItemClass::Ore, "Coal", 2, items)
            }
            StainlessSteel => {
                inventory.consume(ItemClass::Ore, "Iron Ore", 3, items);
                inventory.consume(ItemClass::Ore, "Zince", 1, items);
                inventory.consume(ItemClass::Ore, "Coal", 1, items)
            }
            Stellite => {
                inventory.consume(ItemClass::Ore, "Iron Ore", 2, items);
                inventory.consume(ItemClass::Ore, "Tungsten Ore", 2, items);
                inventory.consume(ItemClass::Ore, "Coal", 1, items)
            }
        }
    }

    pub fn produce_results_for(
        product: SmeltingType,
        player: &mut Player,
        rng: &mut Rng,
        update_tx: Option<&GameUpdateSender>,
    ) -> (ItemClass, String) {
        let smelting_product = SMELTING_PRODUCTS[&product];
        player.increment_xp(
            Smelting,
            smelting_product.smelting_xp as u64,
            rng,
            update_tx,
        );

        (ItemClass::Material, format!("{} Bar", product.to_string()))
    }
}
