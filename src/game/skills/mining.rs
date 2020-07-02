use super::*;
use ItemClass::*;
use OreType::*;

lazy_static! {
    static ref MINE_PRODUCTS: HashMap<OreType, MiningSkill> = {
        let mut m = HashMap::new();
        m.insert(OreType::Dirt, MiningSkill::new(1, 3));
        m.insert(OreType::Sand, MiningSkill::new(1, 2));
        m.insert(OreType::Stone, MiningSkill::new(1, 4));
        m.insert(OreType::Tin, MiningSkill::new(2, 5));
        m.insert(OreType::Copper, MiningSkill::new(3, 6));
        m.insert(OreType::Coal, MiningSkill::new(6, 8));
        m.insert(OreType::Lead, MiningSkill::new(9, 9));
        m.insert(OreType::Cinnabar, MiningSkill::new(12, 10));
        m.insert(OreType::Iron, MiningSkill::new(18, 20));
        m.insert(OreType::Tungsten, MiningSkill::new(36, 50));
        m.insert(OreType::Cobalt, MiningSkill::new(15, 15));
        m.insert(OreType::Nickel, MiningSkill::new(21, 25));
        m.insert(OreType::Gold, MiningSkill::new(24, 30));
        m.insert(OreType::Bauxite, MiningSkill::new(27, 35));
        m.insert(OreType::Silver, MiningSkill::new(30, 40));
        m.insert(OreType::Zinc, MiningSkill::new(33, 45));
        m.insert(OreType::Platinum, MiningSkill::new(39, 55));
        m.insert(OreType::Titanium, MiningSkill::new(42, 60));
        m.insert(OreType::Mythral, MiningSkill::new(45, 65));
        m
    };
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum OreType {
    Dirt = 1,
    Sand,
    Stone,
    Tin,
    Copper,
    Coal,
    Lead,
    Cinnabar,
    Iron,
    Tungsten,
    Cobalt,
    Nickel,
    Steel,
    Gold,
    Bauxite,
    Silver,
    Zinc,
    Platinum,
    Titanium,
    Mythral,
}

impl OreType {
    pub fn from(value: i128) -> OreType {
        match value {
            1 => Dirt,
            2 => Sand,
            3 => Stone,
            4 => Tin,
            5 => Copper,
            6 => Coal,
            7 => Lead,
            8 => Cinnabar,
            9 => Iron,
            10 => Tungsten,
            11 => Cobalt,
            12 => Nickel,
            13 => Steel,
            14 => Gold,
            15 => Bauxite,
            16 => Silver,
            17 => Zinc,
            18 => Platinum,
            19 => Titanium,
            20 => Mythral,
            _ => panic!("unknown ore type"),
        }
    }

    pub fn to_ore_product(&self) -> String {
        let product_string = self.to_string();
        match self {
            Dirt | Stone | Sand | Cinnabar | Bauxite => product_string,
            _ => format!("{} Ore", product_string),
        }
    }
}

impl ToString for OreType {
    fn to_string(&self) -> String {
        match self {
            OreType::Dirt => "Dirt",
            OreType::Sand => "Sand",
            OreType::Stone => "Stone",
            OreType::Tin => "Tin",
            OreType::Copper => "Copper",
            OreType::Coal => "Coal",
            OreType::Lead => "Lead",
            OreType::Cinnabar => "Cinnabar",
            OreType::Iron => "Iron",
            OreType::Tungsten => "Tungsten",
            OreType::Cobalt => "Cobalt",
            OreType::Nickel => "Nickel",
            OreType::Steel => "Steel",
            OreType::Gold => "Gold",
            OreType::Bauxite => "Bauxite",
            OreType::Silver => "Silver",
            OreType::Zinc => "Zinc",
            OreType::Platinum => "Platinum",
            OreType::Titanium => "Titanium",
            OreType::Mythral => "Mythral",
        }
        .to_string()
    }
}

pub struct MiningSkill {
    mining_level: u8,
    mining_xp: u8,
}

impl MiningSkill {
    pub fn new(mining_level: u8, mining_xp: u8) -> Self {
        Self {
            mining_level,
            mining_xp,
        }
    }

    pub fn can_produce(product: OreType, level: u8) -> bool {
        let required_level = MINE_PRODUCTS
            .get(&product)
            .expect("unable to find product in MINE_PRODUCTS")
            .mining_level;

        required_level <= level
    }

    pub fn expiration(product: OreType, player: &Player) -> u32 {
        (match product {
            Dirt => 40,
            Sand => 20,
            _ => 60,
        } + player.get_attribute(Attribute::SkillTime(Mining), 0)) as u32
    }

    pub fn produce_results_for(
        product: OreType,
        player: &mut Player,
        _rng: &mut Rng,
    ) -> (ItemClass, String) {
        let xp_gain = MINE_PRODUCTS[&product].mining_xp;
        player.increment_xp(Mining, xp_gain as u64);

        let description = product.to_ore_product();
        (Ore, description)
    }
}
