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

lazy_static! {
    static ref CONSTRUCTION_SITE_BLUEPRINTS: [Blueprint; 3] = [
        Blueprint::new(6, 50, 0, 20, 5, 0),
        Blueprint::new(18, 250, 0, 64, 32, 16),
        Blueprint::new(32, 1250, 0, 128, 64, 64),
    ];
}

pub struct Blueprint {
    min_level: u8,
    xp_gain: u16,
    hardwood: u16,
    softwood: u16,
    stone: u16,
    rope: u16,
}

impl Blueprint {
    pub fn new(
        min_level: u8,
        xp_gain: u16,
        hardwood: u16,
        softwood: u16,
        stone: u16,
        rope: u16,
    ) -> Self {
        Self {
            min_level,
            xp_gain,
            hardwood,
            softwood,
            stone,
            rope,
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ConstructionSiteType {
    SmallSite = 0,
    MediumSite,
    LargeSite,
}

pub enum ConstructionBuildSiteSkill {}

impl ConstructionBuildSiteSkill {
    pub fn is_buildable_at(_x: i32, _y: i32) -> bool {
        true
    }

    pub fn largest_size_of_construction_site_that_can_be_built(
        inventory: &Inventory,
    ) -> Option<u8> {
        for size in 1..=3 {
            if Self::has_supplies_for_construction_site_of_size(4 - size, inventory) {
                return Some(4 - size);
            }
        }
        None
    }

    pub fn has_supplies_for_construction_site_of_size(size: u8, inventory: &Inventory) -> bool {
        let blueprint = &CONSTRUCTION_SITE_BLUEPRINTS[(size - 1) as usize];

        inventory.has_sufficient(Material, "Hardwood Plank", blueprint.hardwood)
            && inventory.has_sufficient(Material, "Softwood Plank", blueprint.softwood)
            && inventory.has_sufficient(Ore, "Stone", blueprint.stone)
            && inventory.has_sufficient(Material, "Rope", blueprint.rope)
    }

    pub fn can_produce(selection: u8, level: u8, inventory: &Inventory) -> bool {
        let blueprint = &CONSTRUCTION_SITE_BLUEPRINTS[(selection - 1) as usize];

        level >= blueprint.min_level
            && inventory.has_sufficient(Material, "Hardwood Plank", blueprint.hardwood)
            && inventory.has_sufficient(Material, "Softwood Plank", blueprint.softwood)
            && inventory.has_sufficient(Ore, "Stone", blueprint.stone)
            && inventory.has_sufficient(Material, "Rope", blueprint.rope)
    }

    pub fn expiration(selection: u8, player: &Player) -> u32 {
        let base_timer = (match selection {
            1 => 300,
            2 => 600,
            3 => 1200,
            _ => 0,
        }) as u32;

        (base_timer as i32 + player.get_attribute(Attribute::SkillTime(Construction), 0) as i32)
            as u32
    }

    pub fn consume_from_inventory_for(
        site_type: ConstructionSiteType,
        player: &mut Player,
        inventories: &mut InventoryList,
        items: &mut ItemList,
    ) {
        let inventory = inventories
            .get_mut(&player.inventory_id())
            .expect("unable to get player's inventory.");

        let blueprint = &CONSTRUCTION_SITE_BLUEPRINTS[site_type as usize];

        inventory.consume(Material, "Hardwood Plank", blueprint.hardwood, items);
        inventory.consume(Material, "Softwood Plank", blueprint.softwood, items);
        inventory.consume(Ore, "Stone", blueprint.stone, items);
        inventory.consume(Material, "Rope", blueprint.rope, items);
    }

    pub fn produce_results_for(
        site_type: ConstructionSiteType,
        player: &mut Player,
        rng: &mut Rng,
        update_tx: Option<&GameUpdateSender>,
        command_tx: Option<CommandSender>,
    ) {
        let blueprint = &CONSTRUCTION_SITE_BLUEPRINTS[site_type as usize];
        let xp_gain = blueprint.xp_gain as u64;

        let description: String;
        let size: u8;

        match site_type {
            ConstructionSiteType::SmallSite => {
                description = "A Small Construction Site".into();
                size = 1
            }
            ConstructionSiteType::MediumSite => {
                description = "A Construction Site".into();
                size = 2
            }
            ConstructionSiteType::LargeSite => {
                description = "A Large, Hectic Construction Site".into();
                size = 3
            }
        }

        Command::send(
            command_tx,
            Command::SpawnFacility(
                player.x,
                player.y,
                FacilityClass::ConstructionSite,
                description,
                format!("property: size => {}", size),
            ),
        );

        player.increment_xp(Construction, xp_gain, rng, update_tx);
    }
}
