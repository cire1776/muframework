use super::*;
use ItemClass::*;
use ProduceType::*;

lazy_static! {
    static ref HARVESTING_PRODUCTS: HashMap<ProduceType, HarvestingSkill> = {
        let mut m = HashMap::new();
        m.insert(Radish, HarvestingSkill::new(0, 15, 1));
        m.insert(Carrot, HarvestingSkill::new(0, 15, 1));
        m.insert(Beet, HarvestingSkill::new(0, 15, 2));
        m.insert(Onion, HarvestingSkill::new(0, 15, 2));
        m.insert(Cabbage, HarvestingSkill::new(1, 15, 3));
        m.insert(Corn, HarvestingSkill::new(2, 15, 4));
        m.insert(Potato, HarvestingSkill::new(3, 15, 4));
        m.insert(JalapenoPepper, HarvestingSkill::new(4, 15, 5));
        m.insert(Strawberry, HarvestingSkill::new(4, 15, 6));
        m.insert(Hop, HarvestingSkill::new(5, 20, 7));
        m.insert(Spinach, HarvestingSkill::new(6, 25, 8));
        m.insert(Broccoli, HarvestingSkill::new(7, 30, 9));
        m.insert(Asparagus, HarvestingSkill::new(8, 35, 10));
        m.insert(Wheat, HarvestingSkill::new(9, 40, 10));
        m.insert(Tomato, HarvestingSkill::new(10, 45, 11));
        m.insert(Parsnip, HarvestingSkill::new(11, 45, 12));
        m.insert(Turnip, HarvestingSkill::new(13, 45, 15));
        m.insert(Eggplant, HarvestingSkill::new(15, 45, 20));
        m.insert(Cucumber, HarvestingSkill::new(20, 45, 25));
        m.insert(Pumpkin, HarvestingSkill::new(25, 45, 30));
        m.insert(Sugarcane, HarvestingSkill::new(30, 45, 40));
        m.insert(Watermelon, HarvestingSkill::new(35, 45, 50));
        m.insert(GreenPepper, HarvestingSkill::new(40, 45, 60));
        m.insert(BlackPepper, HarvestingSkill::new(45, 45, 70));

        m
    };
}

// order my change, numbering cannot.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ProduceType {
    Asparagus = 1,
    Beet = 2,
    BlackPepper = 3,
    Broccoli = 4,
    Cabbage = 5,
    Carrot = 6,
    Corn = 7,
    Cucumber = 8,
    Eggplant = 9,
    GreenPepper = 10,
    Hop = 11,
    JalapenoPepper = 12,
    Onion = 13,
    Parsnip = 14,
    Potato = 15,
    Pumpkin = 16,
    Radish = 17,
    Spinach = 18,
    Strawberry = 19,
    Sugarcane = 20,
    Tomato = 21,
    Turnip = 22,
    Watermelon = 23,
    Wheat = 24,
}

impl ProduceType {
    pub fn from(value: i128) -> ProduceType {
        match value {
            1 => Asparagus,
            2 => Beet,
            3 => BlackPepper,
            4 => Broccoli,
            5 => Cabbage,
            6 => Carrot,
            7 => Corn,
            8 => Cucumber,
            9 => Eggplant,
            10 => GreenPepper,
            11 => Hop,
            12 => JalapenoPepper,
            13 => Onion,
            14 => Parsnip,
            15 => Potato,
            16 => Pumpkin,
            17 => Radish,
            18 => Spinach,
            19 => Strawberry,
            20 => Sugarcane,
            21 => Tomato,
            22 => Turnip,
            23 => Watermelon,
            24 => Wheat,
            _ => panic!("unknown produce type"),
        }
    }
}

impl ToString for ProduceType {
    fn to_string(&self) -> String {
        match self {
            Asparagus => "Asparagus",
            Beet => "Beet",
            BlackPepper => "Black Pepper",
            Broccoli => "Broccoli",
            Cabbage => "Cabbage",
            Carrot => "Carrot",
            Corn => "Corn",
            Cucumber => "Cucumber",
            Eggplant => "Eggplant",
            GreenPepper => "Green Pepper",
            Hop => "Hop",
            JalapenoPepper => "Jalapeno Pepper",
            Onion => "Onion",
            Parsnip => "Parsnip",
            Potato => "Potato",
            Pumpkin => "Pumpkin",
            Radish => "Radish",
            Spinach => "Spinach",
            Strawberry => "Strawberry",
            Sugarcane => "Sugarcane",
            Tomato => "Tomato",
            Turnip => "Turnip",
            Watermelon => "Watermelon",
            Wheat => "Wheat",
        }
        .to_string()
    }
}
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct HarvestingSkill {
    minimum_level: u8,
    maximum_level: u8,
    xp_gain: u8,
}

impl HarvestingSkill {
    pub fn new(minimum_level: u8, maximum_level: u8, xp_gain: u8) -> Self {
        Self {
            minimum_level,
            maximum_level,
            xp_gain,
        }
    }
    pub fn can_produce(product: ProduceType, player: &Player) -> bool {
        let rules = HARVESTING_PRODUCTS[&product];
        let player_level = player.get_level_for(Harvesting);

        rules.minimum_level <= player_level && rules.maximum_level >= player_level
    }

    pub fn is_exhasuted(facility: &Facility) -> bool {
        facility.get_property("quantity") == 0
    }

    pub fn expiration(_product: ProduceType, player: &Player) -> u32 {
        (60 + player.get_attribute(Attribute::SkillTime(Harvesting), 0)) as u32
        // 60
    }

    pub fn produce_results_for(
        product: ProduceType,
        player: &mut Player,
        facility: &mut Facility,
        _rng: &mut Rng,
    ) -> (ItemClass, String) {
        player.increment_xp(Harvesting, HARVESTING_PRODUCTS[&product].xp_gain as u64);

        facility.decrement_property("quantity");
        let produce = facility.get_property("produce");
        (Food, ProduceType::from(produce).to_string())
    }
}
