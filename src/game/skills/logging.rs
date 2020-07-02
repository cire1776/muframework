use super::*;
use game::command::facility_commands::tree_commands::TreeType;
use ItemClass::*;

pub struct LoggingSkill {}

impl LoggingSkill {
    pub fn expiration(_product: TreeType, player: &Player) -> u32 {
        (60 + player.get_attribute(Attribute::SkillTime(Logging), 0)) as u32
    }

    pub fn produce_results_for(
        product: TreeType,
        player: &mut Player,
        _rng: &mut Rng,
    ) -> (ItemClass, String) {
        let xp_gain = match product {
            TreeType::Pine => 5,
            TreeType::Apple | TreeType::Olive => 6,
            TreeType::Oak => 8,
        };

        player.increment_xp(Logging, xp_gain);

        let wood_type = match product {
            TreeType::Apple | TreeType::Olive | TreeType::Oak => "Hardwood Log",
            TreeType::Pine => "Softwood Log",
        };

        (Material, wood_type.into())
    }
}
