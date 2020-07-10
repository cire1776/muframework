use super::*;
use game::command::facility_commands::well_commands::WellType;
pub enum EngineeringSkill {}

impl EngineeringSkill {
    pub fn can_produce(level: u8, facility: &Facility) -> bool {
        let level_of_well = Self::get_well_level(facility);
        level_of_well.1 <= level
    }

    pub fn expiration(player: &Player) -> u32 {
        (60 + player.get_attribute(Attribute::SkillTime(Engineering), 0)) as u32
    }

    pub fn produce_results_for(
        player: &mut Player,
        facility: &mut Facility,
        rng: &mut Rng,
        update_tx: Option<&GameUpdateSender>,
    ) -> WellType {
        facility.increment_property("depth");
        let well_level = Self::get_well_level(facility);

        let xp_gain = well_level.2;
        player.increment_xp(Engineering, xp_gain as u64, rng, update_tx);

        let water_chance = facility.get_property("chance_of_hitting_water");
        if rng.succeeds(0, water_chance, "water_chance") {
            // bonus xp gain for completing water well
            player.increment_xp(Engineering, well_level.3 as u64, rng, update_tx);
            return WellType::Water;
        }

        let oil_chance = facility.get_property("chance_of_hitting_oil");
        if rng.succeeds(0, oil_chance, "oil_chance") {
            // bonus xp gain for completing oil well
            player.increment_xp(Engineering, well_level.4 as u64, rng, update_tx);
            return WellType::Oil;
        }

        let bedrock_chance = facility.get_property("chance_of_hitting_bedrock");
        if rng.succeeds(0, bedrock_chance, "bedrock_chance") {
            // bonus xp gain for striking bedrock
            player.increment_xp(Engineering, well_level.5 as u64, rng, update_tx);
            return WellType::Bedrock;
        }

        WellType::Dry
    }

    fn get_well_level(facility: &Facility) -> (u8, u8, u8, u16, u16, u16) {
        let depth = facility.get_property("depth");

        if depth < 300 {
            (1, 1, 10, 100, 1000, 15000)
        } else if depth < 650 {
            (2, 20, 18, 200, 2000, 10000)
        } else if depth < 1000 {
            (3, 30, 23, 300, 3000, 5000)
        } else {
            (0, 46, 0, 0, 0, 0)
        }
    }
}
