use super::*;

pub mod smelting;
pub use smelting::*;

pub mod cooking;
pub use cooking::*;

pub mod logging;
pub use logging::*;

pub mod construction;
pub use construction::*;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Skill {
    Alchemy,
    Character,
    Combat,
    Construction,
    Cooking,
    Engineering,
    Fishing,
    Harvesting,
    Lockpicking,
    Locksmithing,
    Logging,
    Mining,
    Smelting,
    Smithing,
}

impl Skill {
    pub fn from_string<S: ToString>(string: S) -> Skill {
        match &string.to_string()[..] {
            "alchemy" => Alchemy,
            "character" => Character,
            "combat" => Combat,
            "construction" => Construction,
            "cooking" => Cooking,
            "engineering" => Engineering,
            "fishing" => Fishing,
            "harvesting" => Harvesting,
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
