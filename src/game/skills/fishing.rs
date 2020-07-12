use super::*;

use game::command::facility_commands::fishing_spot_commands::FishType;
use FishType::*;
use ItemClass::*;

lazy_static! {
    static ref FISHING_SPECIFICATIONS: HashMap<FishType, FishingSkill> = {
        let mut m = HashMap::new();
        m.insert(Shrimp, FishingSkill::new(1, 45, 3));
        m.insert(Frog, FishingSkill::new(1, 45, 3));
        m.insert(Mackeral, FishingSkill::new(1, 45, 4));
        m.insert(Crab, FishingSkill::new(1, 45, 4));
        m.insert(Catfish, FishingSkill::new(5, 45, 5));
        m.insert(Salmon, FishingSkill::new(5, 45, 5));
        m.insert(Bass, FishingSkill::new(10, 54, 6));
        m.insert(Oyster, FishingSkill::new(10, 45, 6));
        m.insert(Flounder, FishingSkill::new(15, 45, 7));
        m.insert(Haddock, FishingSkill::new(15, 45, 7));
        m.insert(Swordfish, FishingSkill::new(20, 45, 8));
        m.insert(Eel, FishingSkill::new(20, 45, 8));
        m.insert(Sardine, FishingSkill::new(25, 45, 10));
        m.insert(SandbarShark, FishingSkill::new(25, 45, 10));
        m.insert(Pike, FishingSkill::new(30, 45, 15));
        m.insert(Lobster, FishingSkill::new(30, 45, 15));
        m.insert(Tuna, FishingSkill::new(35, 45, 20));
        m.insert(StripedMarlin, FishingSkill::new(35, 45, 20));
        m.insert(Herring, FishingSkill::new(40, 45, 25));
        m.insert(Trout, FishingSkill::new(40, 45, 25));
        m.insert(Snapper, FishingSkill::new(45, 45, 29));
        m.insert(RedTrout, FishingSkill::new(45, 45, 30));
        m.insert(RedHerring, FishingSkill::new(45, 45, 30));
        m.insert(Cod, FishingSkill::new(45, 45, 30));
        m
    };
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum FishingType {
    Net,
    Rod,
    TrapPlacing,
    TrapCollection,
}

#[derive(Debug, Clone)]
pub struct FishingSpotProperties {
    facility: Facility,
}

impl<'a> FishingSpotProperties {
    pub fn new(facility: Facility) -> Self {
        Self { facility }
    }
    pub fn body_of_waster(&self) -> u64 {
        self.facility.get_property("body_of_waster") as u64
    }

    pub fn net_products(&self) -> (FishType, Option<FishType>) {
        let fish_type_1 = FishType::from(self.facility.get_property("net_1_product") as u8);
        let fish_type_2 = FishType::from(self.facility.get_property("net_2_product") as u8);
        (fish_type_1.expect("unable to find fish"), fish_type_2)
    }

    pub fn net_product_chance(&self) -> u8 {
        (100 - self.facility.get_property("net_2_chance")) as u8
    }

    pub fn net_timer(&self) -> u32 {
        self.facility.get_property("net_timer") as u32
    }

    pub fn rod_products(&self) -> (FishType, Option<FishType>) {
        let fish_type_1 = FishType::from(self.facility.get_property("rod_1_product") as u8);
        let fish_type_2 = FishType::from(self.facility.get_property("rod_2_product") as u8);
        (fish_type_1.expect("unable to find fish"), fish_type_2)
    }

    pub fn rod_product_chance(&self) -> u8 {
        (100 - self.facility.get_property("rod_2_chance")) as u8
    }

    pub fn rod_timer(&self) -> u32 {
        self.facility.get_property("rod_timer") as u32
    }

    pub fn trap_products(&self) -> (FishType, Option<FishType>) {
        let fish_type_1 = FishType::from(self.facility.get_property("trap_1_product") as u8);
        let fish_type_2 = FishType::from(self.facility.get_property("trap_2_product") as u8);
        (fish_type_1.expect("unable to find fish"), fish_type_2)
    }

    pub fn trap_product_chance(&self) -> u8 {
        (100 - self.facility.get_property("trap_2_chance")) as u8
    }

    pub fn trap_timer(&self) -> u32 {
        self.facility.get_property("trap_timer") as u32
    }

    pub fn trap_spawn(&self) -> u8 {
        self.facility.get_property("trap_spawn") as u8
    }

    pub fn trap_cooldown(&self) -> u32 {
        self.facility.get_property("trap_cooldown") as u32
    }

    pub fn fish_type(
        product_1: FishType,
        product_2: Option<FishType>,
        product_chance: u8,
        rng: &mut Rng,
    ) -> FishType {
        if product_2.is_none() {
            product_1
        } else {
            if rng.percentile(product_chance, "fish_type") {
                product_1
            } else {
                product_2.unwrap()
            }
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct FishingSkill {
    minimum_level: u8,
    maximum_level: u8,
    xp_gain: u8,
}

impl FishingSkill {
    fn new(minimum_level: u8, maximum_level: u8, xp_gain: u8) -> Self {
        Self {
            minimum_level,
            maximum_level,
            xp_gain,
        }
    }

    pub fn can_produce(product: FishType, player: &Player) -> bool {
        let level = player.get_level_for(Fishing);
        let specs = FISHING_SPECIFICATIONS[&product];

        level >= specs.minimum_level && level <= specs.maximum_level
    }

    pub fn expiration(
        player: &Player,
        fishing_type: FishingType,
        properties: &FishingSpotProperties,
    ) -> u32 {
        let base_time = match fishing_type {
            FishingType::Net => properties.net_timer(),
            FishingType::Rod => properties.rod_timer(),
            FishingType::TrapPlacing | FishingType::TrapCollection => properties.trap_timer(),
        };

        let modifier = player.get_attribute(Attribute::SkillTime(Fishing.into()), 0);

        (base_time as i64 + modifier as i64) as u32
    }

    pub fn produce_results_for(
        fishing_type: FishingType,
        player: &mut Player,
        facility: &mut Facility,
        rng: &mut Rng,
        update_tx: Option<&GameUpdateSender>,
    ) -> Vec<(ItemClass, String)> {
        let properties = FishingSpotProperties::new(facility.clone());
        let spawns = if fishing_type == FishingType::TrapCollection {
            facility.set_property("trap_expiration", 0);
            facility.set_property("is_in_use", 0);

            let max_spawn = facility.get_property("trap_spawn");
            rng.range(1, max_spawn + 1, "trap_spawn")
        } else {
            1
        };

        let (product_1, product_2) = match fishing_type {
            FishingType::Rod => properties.rod_products(),
            FishingType::Net => properties.net_products(),
            FishingType::TrapCollection => properties.trap_products(),
            _ => todo!(),
        };

        let product_chance = match fishing_type {
            FishingType::Rod => properties.rod_product_chance(),
            FishingType::Net => properties.net_product_chance(),
            FishingType::TrapCollection => properties.trap_product_chance(),
            _ => todo!(),
        };

        let mut result = vec![];
        let mut xp_gain: u64 = 0;

        for _ in 0..spawns {
            let fish_type =
                FishingSpotProperties::fish_type(product_1, product_2, product_chance, rng);
            let product = if Self::can_produce(fish_type, player) {
                xp_gain += FISHING_SPECIFICATIONS[&fish_type].xp_gain as u64;
                (Ingredient, fish_type.to_string())
            } else {
                xp_gain += 1;

                if rng.percentile(50, "flotsam_type") {
                    (Ingredient, "Seaweed".to_string())
                } else {
                    (Material, "Driftwood".to_string())
                }
            };
            result.push(product);
        }

        if xp_gain > 0 {
            player.increment_xp(Fishing, xp_gain, rng, update_tx);
        }

        result
    }
}
