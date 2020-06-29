use super::*;
use game::command::facility_commands::tree_commands::TreeType;
use ItemClass::*;

pub struct LoggingSkill {}

impl LoggingSkill {
    pub fn produce_results_for(
        product: TreeType,
        _player: &mut Player,
        _rng: &mut Rng,
    ) -> (ItemClass, String) {
        let wood_type = match product {
            TreeType::Apple | TreeType::Olive | TreeType::Oak => "Hardwood Log",
            TreeType::Pine => "Softwood Log",
        };
        (Material, wood_type.into())
    }
}
