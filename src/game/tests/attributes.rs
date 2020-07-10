use super::*;
use Attribute::*;

#[test]
fn at_level_one_skill_there_is_0_skilltime_bonus() {
    let mut subject = Player::new();

    subject.set_level_for(Cooking, 1, None);

    assert_eq!(subject.get_attribute(Attribute::SkillTime(Cooking), 0), 0)
}

#[test]
fn base_skilltime_is_set_upon_setting_level() {
    let mut subject = Player::new();

    subject.set_level_for(Cooking, 10, None);

    assert_eq!(subject.get_attribute(SkillTime(Cooking), 0), -9)
}

#[test]
fn buffs_are_counted_in_skilltime() {
    let mut subject = Player::new();

    subject.set_level_for(Mining, 1, None);
    subject.add_buff(Attribute::SkillTime(Mining), (-3, 10, BuffTag::Effect));

    assert_eq!(subject.get_attribute(SkillTime(Mining), 5), -3)
}

#[test]
fn buffs_can_expire() {
    let mut subject = Player::new();

    subject.set_level_for(Mining, 1, None);
    subject.add_buff(Attribute::SkillTime(Mining), (-3, 10, BuffTag::Effect));

    assert_eq!(subject.get_attribute(SkillTime(Mining), 11), 0)
}
