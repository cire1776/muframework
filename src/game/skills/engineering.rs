use super::*;
use game::command::facility_commands::well_commands::WellType;
pub enum EngineeringSkill {}

impl EngineeringSkill {
    pub fn can_produce(level: u8, facility: &Facility) -> bool {
        let _depth = facility.get_property("depth");
        let depth = facility.get_property("depth");

        let level_of_well = if depth < 300 {
            (1, 1)
        } else if depth < 650 {
            (2, 20)
        } else if depth < 1000 {
            (3, 30)
        } else {
            (0, 46)
        };

        level_of_well.1 <= level
    }

    pub fn produce_results_for(
        _player: &mut Player,
        facility: &mut Facility,
        rng: &mut Rng,
    ) -> WellType {
        facility.increment_property("depth");

        let water_chance = facility.get_property("chance_of_hitting_water");
        if rng.succeeds(0, water_chance, "water_chance") {
            return WellType::Water;
        }

        let oil_chance = facility.get_property("chance_of_hitting_oil");
        if rng.succeeds(0, oil_chance, "oil_chance") {
            return WellType::Oil;
        }

        let bedrock_chance = facility.get_property("chance_of_hitting_bedrock");
        if rng.succeeds(0, bedrock_chance, "bedrock_chance") {
            return WellType::Bedrock;
        }

        WellType::Dry
    }
}
