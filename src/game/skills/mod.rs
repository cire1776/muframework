use super::*;

pub mod smelting;
pub use smelting::*;

pub mod cooking;
pub use cooking::*;

pub mod logging;
pub use logging::*;

pub mod construction;
pub use construction::*;

pub mod mining;
pub use mining::*;

pub mod engineering;
pub use engineering::*;

pub mod intellectual;
pub use intellectual::*;

pub mod harvesting;
pub use harvesting::*;

pub mod fishing;
pub use fishing::*;

pub mod alchemy;
pub use alchemy::*;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum Skill {
    Alchemy,
    Character,
    Combat,
    Construction,
    Cooking,
    Cultural,
    Engineering,
    Fishing,
    Harvesting,
    Intellectual,
    Lockpicking,
    Locksmithing,
    Logging,
    Mining,
    Smelting,
    Smithing,
}

impl ToString for Skill {
    fn to_string(&self) -> String {
        match self {
            Alchemy => "Alchemy",
            Skill::Character => "Character",
            Combat => "Combat",
            Construction => "Construction",
            Cooking => "Cooking",
            Cultural => "Cultural",
            Engineering => "Engineering",
            Fishing => "Fishing",
            Harvesting => "Harvesting",
            Intellectual => "Intellectual",
            Lockpicking => "Lockpicking",
            Locksmithing => "Locksmithing",
            Logging => "Logging",
            Mining => "Mining",
            Smelting => "Smelting",
            Smithing => "Smithing",
        }
        .to_string()
    }
}
impl Skill {
    pub fn from_string<S: ToString>(string: S) -> Skill {
        match &string.to_string()[..] {
            "alchemy" => Alchemy,
            "character" => Character,
            "combat" => Combat,
            "construction" => Construction,
            "cooking" => Cooking,
            "cultural" => Cultural,
            "engineering" => Engineering,
            "fishing" => Fishing,
            "harvesting" => Harvesting,
            "intellectual" => Intellectual,
            "lockpicking" => Lockpicking,
            "locksmithing" => Locksmithing,
            "logging" => Logging,
            "mining" => Mining,
            "smelting" => Smelting,
            "smithing" => Smithing,
            _ => panic!("unknown skill: {}", string.to_string()),
        }
    }
}
