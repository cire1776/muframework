use super::*;

const LEVEL_REQUIREMENTS: [u32; 44] = [
    35, 70, 140, 280, 560, 1120, 2240, 4480, 8960, 17920, 35840, 50160, 58890, 68300, 78300, 89000,
    100300, 113000, 125000, 138200, 152000, 166700, 181800, 197600, 214000, 231000, 249000, 267000,
    286000, 305000, 325000, 346000, 367000, 389000, 412000, 435000, 459000, 483000, 508000, 533000,
    559000, 586000, 613000, 641000,
];

pub struct Levelling {}

impl Levelling {
    pub fn check_for_levelling(
        player: &mut Player,
        skill: Skill,
        gain: u32,
        rng: &mut Rng,
        update_tx: Option<&GameUpdateSender>,
    ) {
        let current_level = player.get_level_for(skill);

        if current_level == 45 || current_level == 0 {
            return;
        }

        let target = LEVEL_REQUIREMENTS[(current_level - 1) as usize] as i128;
        let roll = rng.range(0, target, "levelling check");

        if roll < gain as i128 {
            player.set_level_for(skill, current_level + 1, update_tx);
        }
    }
}
