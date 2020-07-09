use super::*;
pub use Skill::*;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize)]
pub enum BuffTag {
    None,
    Equipment(u64),
    Inventory(u64),
    Race(String),
    Nationality(String),
    Guild(String),
    Base,
    Effect,
    Level(Skill),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Deserialize, Serialize)]
pub enum Attribute {
    SkillTime(Skill),
    SkillChance(Skill),
    SkillLevel(Skill),
    Fortune,
    SpellCastPeriod,
    SpellDamage,
    Attack,
    Defense,
    MaxHP,
    MaxMP,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Deserialize, Serialize)]
pub enum AttributeBuff {
    SkillTime(Skill, i8, u128),
    SkillChance(Skill, i8, u128),
    SkillLevel(Skill, i8, u128),
    Fortune(i8, u128),
    SpellCastPeriod(i8, u128),
    SpellDamage(i8, u128),
    Attack(i8, u128),
    Defense(i8, u128),
    MaxHP(i8, u128),
    MaxMP(i8, u128),
}

impl AttributeBuff {
    pub fn to_attribute_and_buff(&self, tag: BuffTag) -> (Attribute, Buff) {
        match self {
            AttributeBuff::SkillTime(label, value, expiration) => (
                Attribute::SkillTime(label.clone()),
                (*value, *expiration, tag),
            ),
            AttributeBuff::SkillChance(label, value, expiration) => (
                Attribute::SkillChance(label.clone()),
                (*value, *expiration, tag),
            ),
            AttributeBuff::SkillLevel(label, value, expiration) => (
                Attribute::SkillLevel(label.clone()),
                (*value, *expiration, tag),
            ),
            AttributeBuff::Fortune(value, expiration) => {
                (Attribute::Fortune, (*value, *expiration, tag))
            }
            AttributeBuff::SpellCastPeriod(value, expiration) => {
                (Attribute::SpellCastPeriod, (*value, *expiration, tag))
            }
            AttributeBuff::SpellDamage(value, expiration) => {
                (Attribute::SpellDamage, (*value, *expiration, tag))
            }
            AttributeBuff::Attack(value, expiration) => {
                (Attribute::Attack, (*value, *expiration, tag))
            }
            AttributeBuff::Defense(value, expiration) => {
                (Attribute::Defense, (*value, *expiration, tag))
            }
            AttributeBuff::MaxHP(value, expiration) => {
                (Attribute::MaxHP, (*value, *expiration, tag))
            }
            AttributeBuff::MaxMP(value, expiration) => {
                (Attribute::MaxMP, (*value, *expiration, tag))
            }
        }
    }
}

pub type Buff = (i8, u128, BuffTag);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AttributeList {
    attributes: HashMap<Attribute, Vec<Buff>>,
}

impl AttributeList {
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }

    pub fn add(&mut self, attribute: Attribute, buff: Buff) {
        let possible_attribute_set = &mut self.attributes.get_mut(&attribute);

        match possible_attribute_set {
            Some(ref mut attribute_set) => attribute_set.push(buff),
            None => {
                self.attributes.insert(attribute, vec![buff]);
                ()
            }
        }
    }
    pub fn get(&self, index: &Attribute, current_time: u128) -> i8 {
        let possible_attribute_set = &self.attributes.get(index);
        match possible_attribute_set {
            Some(ref attribute_set) => {
                let new_set: Vec<Buff> = attribute_set
                    .iter()
                    .filter(|a| a.1 == 0 || a.1 > current_time)
                    .map(|a| a.clone())
                    .collect();
                new_set.iter().fold(0, |accum, a| accum + a.0)
            }
            None => 0,
        }
    }

    pub fn remove(&mut self, tag: BuffTag) {
        for (_attribute, set) in self.attributes.iter_mut() {
            *set = set
                .iter()
                .filter(|a| a.2 != tag)
                .map(|a| a.clone())
                .collect()
        }
    }
}

#[cfg(test)]
mod attribute_list {
    use super::*;
    use Attribute::*;
    use BuffTag::*;

    #[test]
    fn add_adds_a_buff_to_an_empty_attribute() {
        let mut subject = AttributeList::new();

        subject.add(SkillTime(Fishing), (-3, 0, Equipment(1159)));

        assert_eq!(subject.attributes.len(), 1);
    }

    #[test]
    fn add_adds_a_buff_to_a_preexisting_attribute() {
        let mut subject = AttributeList::new();
        subject.add(SkillTime(Fishing.into()), (-3, 0, Equipment(1159)));
        subject.add(SkillTime(Fishing.into()), (-3, 30000, Effect));

        assert_eq!(subject.attributes.len(), 1);
        assert_eq!(subject.attributes[&SkillTime(Fishing.into())].len(), 2);
    }

    #[test]
    fn get_returns_0_if_not_attribute_has_been_set() {
        let subject = AttributeList::new();

        assert_eq!(subject.get(&SkillChance(Fishing), 0), 0);
    }

    #[test]
    fn get_returns_3_if_attribute_has_been_set_for_3() {
        let mut subject = AttributeList::new();

        subject.add(SkillChance(Fishing), (3, 0, Base));

        assert_eq!(subject.get(&SkillChance(Fishing), 0), 3);
    }

    #[test]
    fn get_returns_5_if_two_attributes_have_been_set_for_5() {
        let mut subject = AttributeList::new();

        subject.add(SkillChance(Fishing), (3, 0, Base));
        subject.add(SkillChance(Fishing), (2, 0, Base));

        assert_eq!(subject.get(&SkillChance(Fishing), 0), 5);
    }

    #[test]
    fn remove_removes_all_buffs_with_given_tag() {
        let mut subject = AttributeList::new();

        subject.add(Fortune, (2, 0, Inventory(112)));
        subject.add(Fortune, (5, 0, Inventory(112)));
        subject.add(Fortune, (5, 0, Base));

        subject.remove(Inventory(112));

        assert_eq!(subject.get(&Fortune, 0), 5);
    }
}
