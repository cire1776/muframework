use super::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BuffTag {
    Equipment(u64),
    Inventory(u64),
    Race(String),
    Nationality(String),
    Guild(String),
    Base,
    Effect,
    Level,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Attribute {
    SkillTime(String),
    SkillChance(String),
    Fortune,
    SpellCastPeriod,
    SpellDamage,
    Defense,
    MaxHP,
    MaxMP,
}

pub type Buff = (i8, u128, BuffTag);

#[derive(Debug, Clone)]
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
    pub fn get(&mut self, index: &Attribute, current_time: u128) -> i8 {
        let possible_attribute_set = &self.attributes.get(index);
        match possible_attribute_set {
            Some(ref attribute_set) => {
                let new_set: Vec<Buff> = attribute_set
                    .iter()
                    .filter(|a| a.1 == 0 || a.1 > current_time)
                    .map(|a| a.clone())
                    .collect();
                let result = new_set.iter().fold(0, |accum, a| accum + a.0);

                self.attributes.insert(index.clone(), new_set);

                result
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

        subject.add(SkillTime("fishing".into()), (-3, 0, Equipment(1159)));

        assert_eq!(subject.attributes.len(), 1);
    }

    #[test]
    fn add_adds_a_buff_to_a_preexisting_attribute() {
        let mut subject = AttributeList::new();
        subject.add(SkillTime("fishing".into()), (-3, 0, Equipment(1159)));
        subject.add(SkillTime("fishing".into()), (-3, 30000, Effect));

        assert_eq!(subject.attributes.len(), 1);
        assert_eq!(subject.attributes[&SkillTime("fishing".into())].len(), 2);
    }

    #[test]
    fn get_returns_0_if_not_attribute_has_been_set() {
        let mut subject = AttributeList::new();

        assert_eq!(subject.get(&SkillChance("bogus_skill".into()), 0), 0);
    }

    #[test]
    fn get_returns_3_if_attribute_has_been_set_for_3() {
        let mut subject = AttributeList::new();

        subject.add(SkillChance("bogus_skill".into()), (3, 0, Base));

        assert_eq!(subject.get(&SkillChance("bogus_skill".into()), 0), 3);
    }

    #[test]
    fn get_returns_5_if_two_attributes_have_been_set_for_5() {
        let mut subject = AttributeList::new();

        subject.add(SkillChance("bogus_skill".into()), (3, 0, Base));
        subject.add(SkillChance("bogus_skill".into()), (2, 0, Base));

        assert_eq!(subject.get(&SkillChance("bogus_skill".into()), 0), 5);
    }

    #[test]
    fn get_removes_expired_buffs() {
        let mut subject = AttributeList::new();

        subject.add(SkillChance("bogus_skill".into()), (3, 30000, Base));
        subject.add(SkillChance("bogus_skill".into()), (2, 0, Base));

        assert_eq!(subject.get(&SkillChance("bogus_skill".into()), 31000), 2);
        assert_eq!(subject.attributes.len(), 1);
        assert_eq!(
            subject.attributes[&SkillChance("bogus_skill".into())].len(),
            1
        );
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
