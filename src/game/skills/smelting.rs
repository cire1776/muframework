use super::*;
use SmeltingSkill::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SmeltingSkill {
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

impl ToString for SmeltingSkill {
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

impl SmeltingSkill {
    pub fn products() -> Vec<(SmeltingSkill, u8)> {
        vec![
            (Tin, 1),             // simple
            (Copper, 2),          // simple
            (Bronze, 4),          // 50/50 compound
            (Lead, 6),            // simple
            (Mercury, 9),         // simple
            (Iron, 12),           // simple
            (Tungsten, 12),       // simple
            (Cobalt, 15),         // simple
            (Nickel, 18),         // simple
            (Steel, 21),          // 3 steel 1 coal + fuel
            (Gold, 24),           // mercury compound
            (Aluminum, 27),       // simple
            (Silver, 30),         // mercury compound
            (Zinc, 33),           // simple
            (Platinum, 36),       // mercury compound
            (StainlessSteel, 39), // zinc compound
            (Stellite, 40),       // 50/50 compound
            (Titanium, 43),       // simple
            (Mythral, 45),        // simple
        ]
    }
    pub fn products_for_player_level(player: &Player) -> Vec<String> {
        let level = std::cmp::max(
            1,
            player.get_attribute(Attribute::SkillLevel(Smelting.into()), 0),
        ) as u8;

        let products = Self::products()
            .iter()
            .take_while(|p| p.1 <= level)
            .map(|p| p.0.to_string())
            .collect();

        products
    }

    pub fn can_produce(product: SmeltingSkill, inventory: &Inventory) -> bool {
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

                if !inventory.has_sufficient(ItemClass::Material, "Coal", 1) {
                    return false;
                }
                true
            }
            Steel => {
                if !inventory.has_sufficient(ItemClass::Ore, "Iron Ore", 3) {
                    return false;
                }

                if !inventory.has_sufficient(ItemClass::Material, "Coal", 2) {
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

                if !inventory.has_sufficient(ItemClass::Material, "Coal", 1) {
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

                if !inventory.has_sufficient(ItemClass::Material, "Coal", 1) {
                    return false;
                }
                true
            }
        }
    }

    pub fn consume_from_inventory_for(
        product: SmeltingSkill,
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
                inventory.consume(ItemClass::Material, "Coal", 1, items)
            }
            Steel => {
                inventory.consume(ItemClass::Ore, "Iron Ore", 3, items);
                inventory.consume(ItemClass::Material, "Coal", 2, items)
            }
            StainlessSteel => {
                inventory.consume(ItemClass::Ore, "Iron Ore", 3, items);
                inventory.consume(ItemClass::Ore, "Zince", 1, items);
                inventory.consume(ItemClass::Material, "Coal", 1, items)
            }
            Stellite => {
                inventory.consume(ItemClass::Ore, "Iron Ore", 2, items);
                inventory.consume(ItemClass::Ore, "Tungsten Ore", 2, items);
                inventory.consume(ItemClass::Material, "Coal", 1, items)
            }
        }
    }
}
