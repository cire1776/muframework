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
    static ref CONSTRUCTION_SITE_BLUEPRINTS: [SiteBlueprint; 3] = [
        SiteBlueprint::new(10, 50, 0, 20, 5, 0),
        SiteBlueprint::new(18, 250, 0, 64, 32, 16),
        SiteBlueprint::new(32, 1250, 0, 128, 64, 64),
    ];
}
lazy_static! {
    static ref FACILITY_BLUEPRINTS: HashMap<FacilityClass, Blueprint> = {
        let mut m = HashMap::new();
        m.insert(
            FacilityClass::Firepit,
            Blueprint::new(
                ConstructionSiteType::SmallSite,
                "Firepit",
                10,
                5,
                0,
                0,
                16,
                0,
            ),
        );
        m.insert(
            FacilityClass::Lumbermill,
            Blueprint::new(
                ConstructionSiteType::SmallSite,
                "Lumbermill",
                12,
                5,
                0,
                64,
                16,
                0,
            ),
        );
        m.insert(
            FacilityClass::ClosedChest,
            Blueprint::new(
                ConstructionSiteType::SmallSite,
                "Chest",
                13,
                5,
                128,
                0,
                0,
                0,
            ),
        );
        m.insert(
            FacilityClass::Well,
            Blueprint::new(ConstructionSiteType::SmallSite, "Well", 14, 5, 0, 64, 64, 0),
        );
        m.insert(
            FacilityClass::FruitPress,
            Blueprint::new(
                ConstructionSiteType::MediumSite,
                "Fruit Press",
                18,
                10,
                32,
                128,
                256,
                32,
            ),
        );
        m
    };
}

pub struct SiteBlueprint {
    min_level: u8,
    xp_gain: u16,
    hardwood: u16,
    softwood: u16,
    stone: u16,
    rope: u16,
}

impl SiteBlueprint {
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

pub struct Blueprint {
    site_type: ConstructionSiteType,
    label: &'static str,
    min_level: u8,
    xp_gain: u16,
    hardwood: u16,
    softwood: u16,
    stone: u16,
    rope: u16,
}

impl Blueprint {
    pub fn new(
        site_type: ConstructionSiteType,
        label: &'static str,
        min_level: u8,
        xp_gain: u16,
        hardwood: u16,
        softwood: u16,
        stone: u16,
        rope: u16,
    ) -> Self {
        Self {
            site_type,
            label,
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
    pub fn available_blueprints_for(player: &Player, facility: &Facility) -> Vec<String> {
        let items = Self::raw_available_blueprints_for(player, facility);
        items.iter().map(|i| i.3.clone()).collect()
    }

    fn raw_available_blueprints_for(
        player: &Player,
        facility: &Facility,
    ) -> Vec<(u8, FacilityClass, &'static Blueprint, String)> {
        let level = player.get_level_for(Construction);
        let size = match facility.get_property("size") {
            1 => ConstructionSiteType::SmallSite,
            2 => ConstructionSiteType::MediumSite,
            3 => ConstructionSiteType::LargeSite,
            _ => ConstructionSiteType::SmallSite,
        };
        let mut items: Vec<(u8, FacilityClass, &'static Blueprint, String)> = FACILITY_BLUEPRINTS
            .iter()
            .filter(|(_, b)| b.min_level <= level && b.site_type == size)
            .map(|(k, b)| (b.min_level, *k, b, b.label.to_string()))
            .collect();
        items.sort_by(|a, b| a.0.cmp(&b.0));
        items
    }

    pub fn set_blueprint_for(selection: u8, player: &Player, facility: &mut Facility) {
        let blueprints = Self::raw_available_blueprints_for(player, facility);
        let class = blueprints[(selection - 1) as usize].1;

        let blueprint = &FACILITY_BLUEPRINTS[&class];
        facility.set_property("blueprint", selection as i8);

        facility.set_property("hardwood", blueprint.hardwood);
        facility.set_property("softwood", blueprint.softwood);
        facility.set_property("stone", blueprint.stone);
        facility.set_property("rope", blueprint.rope);
    }

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

pub struct ConstructionAddSkill {}

impl ConstructionAddSkill {
    pub fn can_produce(player: &Player, facility: &Facility) -> bool {
        if facility.get_property("blueprint") != 0 && player.is_endorsed_with(":wants_to_add") {
            let component = player.get_endorsement_component(":wants_to_add");
            if let Some(component) = component {
                facility.get_property(component) > 0
            } else {
                false
            }
        } else {
            false
        }
    }

    pub fn expiration(player: &Player) -> u32 {
        let base_timer = 60;

        (base_timer as i32 + player.get_attribute(Attribute::SkillTime(Construction), 0) as i32)
            as u32
    }

    pub fn consume_from_inventory_for(
        player: &mut Player,
        facility: &mut Facility,
        inventories: &mut InventoryList,
        items: &mut ItemList,
    ) {
        let inventory = inventories
            .get_mut(&player.inventory_id())
            .expect("unable to get player's inventory.");
        let component = player.get_endorsement_component(":wants_to_add");

        let class: ItemClass;
        let description: String;
        let property_name: String;

        if component.is_none() {
            return;
        }

        match &component.unwrap()[..] {
            "stone" => {
                class = Ore;
                description = "Stone".into();
                property_name = "stone".into();
            }
            "softwood" => {
                class = Material;
                description = "Softwood Plank".into();
                property_name = "softwood".into();
            }
            "hardwood" => {
                class = Material;
                description = "Hardwood Plank".into();
                property_name = "hardwood".into();
            }
            _ => panic!("unknown component: {:?}", component),
        }

        inventory.consume(class, description, 1, items);

        facility.decrement_property(property_name);
    }

    pub fn produce_results_for(
        player: &mut Player,
        facility: &mut Facility,
        inventories: &mut InventoryList,
        rng: &mut Rng,
        update_tx: Option<&GameUpdateSender>,
        _command_tx: Option<CommandSender>,
    ) {
        let selection = facility.get_property("blueprint");

        let blueprints = ConstructionBuildSiteSkill::raw_available_blueprints_for(player, facility);
        let blueprint = &blueprints[(selection - 1) as usize];

        let xp_gain = blueprint.2.xp_gain;

        player.increment_xp(Construction, xp_gain as u64, rng, update_tx);

        let hardwood_left = facility.get_property("hardwood") != 0;
        let softwood_left = facility.get_property("softwood") != 0;
        let stone_left = facility.get_property("stone") != 0;
        let rope_left = facility.get_property("rope") != 0;

        if hardwood_left || softwood_left || stone_left || rope_left {
            return;
        }

        let new_facility_class = blueprint.1;

        let variant: u8 = 0;
        let description: String;

        match new_facility_class {
            FacilityClass::Firepit => description = "A Firepit".into(),
            FacilityClass::Lumbermill => description = "A Simple Lumbermill".into(),
            FacilityClass::ClosedChest => description = "A Chest".into(),
            FacilityClass::Well => description = "A Well".into(),
            _ => todo!("{:?}", new_facility_class),
        }

        facility.class = new_facility_class;
        facility.setup_inventory(inventories);

        match new_facility_class {
            FacilityClass::Lumbermill => facility.set_property("chance_of_breakage", 10_000),
            FacilityClass::Well => {
                facility.set_property("chance_of_hitting_oil", 10_000);
                facility.set_property("chance_of_hitting_water", 100);
                facility.set_property("chance_of_hitting_bedrock", 50_000);
                facility.set_property("chance_of_drying_up", 5_000);
            }
            _ => {}
        }

        GameUpdate::send(
            update_tx,
            FacilityUpdated {
                id: facility.id,
                description,
                class: new_facility_class,
                variant,
            },
        );
    }
}
