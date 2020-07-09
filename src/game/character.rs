use super::*;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub id: u64,
    pub x: i32,
    pub y: i32,
    pub facing: Direction,
    pub character_type: CharacterType,
    pub mounting_points: MountingPointMap,

    #[serde(skip)]
    pub external_inventory: Option<Vec<Item>>,

    endorsements: HashMap<String, u32>,
    endorsement_components: HashMap<String, String>,
    attributes: AttributeList,
    xp: HashMap<Skill, u64>,
}

impl Player {
    pub fn new() -> Player {
        let mut player = Player {
            id: 1,
            x: 0,
            y: 0,
            facing: Direction::Up,
            character_type: CharacterType::Player,
            mounting_points: MountingPointMap::new(),
            external_inventory: None,
            endorsements: HashMap::new(),
            endorsement_components: HashMap::new(),
            attributes: AttributeList::new(),
            xp: HashMap::new(),
        };
        // temporary.  Not sure where this belongs once saving is in place.
        player.endorse_with(":newb");
        player.set_level_for(Smelting, 45);
        player
    }

    pub fn inventory_id(&self) -> u64 {
        self.id
    }

    pub fn is_endorsed_with<S: ToString>(&self, endorsement: S) -> bool {
        self.endorsements.contains_key(&endorsement.to_string())
    }

    pub fn endorse_with<S: ToString>(&mut self, endorsement: S) {
        let endorsement = endorsement.to_string();
        let possible = self.endorsements.get_mut(&endorsement);

        match possible {
            Some(count) => *count += 1,
            None => {
                self.endorsements.insert(endorsement.to_string(), 1);
                ()
            }
        };
    }

    pub fn endorse_component_with<S: ToString>(&mut self, endorsement: S, component: S) {
        let endorsement = endorsement.to_string();

        self.endorse_with(endorsement.clone());

        self.endorsement_components
            .insert(endorsement, component.to_string());
    }

    pub fn unendorse_with<S: ToString>(&mut self, endorsement: S) {
        let endorsement = endorsement.to_string();
        let possible = self.endorsements.get_mut(&endorsement);

        match possible {
            Some(count) => {
                *count -= 1;
                if *count == 0 {
                    self.endorsements.remove(&endorsement);
                }
            }
            None => {}
        };
    }

    pub fn get_endorsement_component<S: ToString>(&self, endorsement: S) -> Option<&String> {
        self.endorsement_components.get(&endorsement.to_string())
    }

    pub fn get_attribute(&self, attribute: Attribute, current_time: u128) -> i8 {
        self.attributes.get(&attribute, current_time)
    }

    pub fn add_buff(&mut self, attribute: Attribute, buff: Buff) {
        self.attributes.add(attribute, buff);
    }

    pub fn remove_buff(&mut self, tag: BuffTag) {
        self.attributes.remove(tag);
    }

    pub fn set_level_for(&mut self, skill: Skill, level: u8) {
        self.remove_buff(BuffTag::Level(skill));

        self.add_buff(
            Attribute::SkillLevel(skill),
            (level as i8, 0, BuffTag::Level(skill)),
        );

        self.set_skilltime_for_level(skill, level);
    }

    fn set_skilltime_for_level(&mut self, skill: Skill, level: u8) {
        self.add_buff(
            Attribute::SkillTime(skill),
            (-(level as i8) + 1, 0, BuffTag::Level(skill)),
        );
    }

    pub fn get_level_for(&self, skill: Skill) -> u8 {
        self.get_attribute(Attribute::SkillLevel(skill), 0) as u8
    }

    pub fn increment_xp(&mut self, skill: Skill, value: u64) {
        let xp = self.xp.get_mut(&skill);
        match xp {
            Some(xp) => *xp += value,
            None => {
                self.xp.insert(skill, value);
            }
        }
    }

    pub fn get_xp(&mut self, skill: Skill) -> u64 {
        let xp = self.xp.get(&skill);
        match xp {
            Some(xp) => *xp,
            None => 0,
        }
    }
}

pub enum CharacterFacing {
    Up,
    Upright,
    Right,
    Downright,
    Down,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Character {
    pub id: u64,
    pub x: i32,
    pub y: i32,
    pub facing: Direction,
    pub character_type: CharacterType,
}

impl Character {
    fn new(id: u64, x: i32, y: i32, facing: Direction, character_type: CharacterType) -> Self {
        Character {
            id,
            x,
            y,
            facing,
            character_type,
        }
    }
    fn read_character(string: &str, re: &Regex) -> Character {
        let captures = re.captures(string).unwrap();
        let character_symbol = captures.get(1).unwrap().clone().as_str().to_string();
        let x = capture_coordinate(&captures, 2);
        let y = capture_coordinate(&captures, 3);

        let character_type = CharacterType::from_symbol(&character_symbol);

        Self::new(NEXT_ID(), x, y, Direction::Up, character_type)
    }

    pub fn read_in_characters(characters: &mut Vec<String>) -> CharacterList {
        let mut result = CharacterList::new();

        characters.reverse();
        characters.pop();

        let re = Regex::new(r"(R|S)\s(\d+)\s*,\s*(\d+)").unwrap();

        for s in characters {
            let character = Self::read_character(s, &re);
            result.add(character);
        }

        result
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub enum CharacterType {
    Player,
    Rat,
    Spider,
}

impl CharacterType {
    fn from_symbol(symbol: &str) -> CharacterType {
        match &symbol[..] {
            "R" => CharacterType::Rat,
            "S" => CharacterType::Spider,
            _ => panic!("unknown symbol"),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CharacterList {
    characters: Vec<Character>,
}

impl CharacterList {
    pub fn new() -> CharacterList {
        CharacterList { characters: vec![] }
    }

    pub fn len(&self) -> usize {
        self.characters.len()
    }

    pub fn add(&mut self, character: Character) {
        self.characters.push(character);
    }
}

pub struct CharacterListIterator<'a> {
    list: &'a CharacterList,
    current: usize,
}

impl<'a> Iterator for CharacterListIterator<'a> {
    type Item = Character;

    fn next(&mut self) -> Option<Character> {
        if self.current >= self.list.len() {
            return None;
        }
        let result = self.list.characters[self.current].clone();
        self.current += 1;
        Some(result)
    }
}
impl CharacterList {
    pub fn iter(&self) -> CharacterListIterator {
        CharacterListIterator {
            list: self,
            current: 0,
        }
    }
}

#[cfg(test)]
mod endorsements {
    use super::*;

    #[test]
    fn is_endorsed_with_returns_false_if_not_endorsed() {
        let subject = Player::new();

        assert!(!subject.is_endorsed_with(":bogus_endorsement"));
    }

    #[test]
    fn is_endorsed_with_returns_true_if_endorsed() {
        let mut subject = Player::new();

        subject.endorse_with(":an_endorsement");

        assert!(subject.is_endorsed_with(":an_endorsement"));
    }

    #[test]
    fn is_endorsed_with_returns_false_after_an_unendorsement() {
        let mut subject = Player::new();

        subject.endorse_with(":an_endorsement");

        subject.unendorse_with(":an_endorsement");

        assert!(!subject.is_endorsed_with(":an_endorsement"));
    }
}

#[cfg(test)]
mod attributes {
    use super::*;
    use game::attributes::Attribute::*;
    use game::attributes::BuffTag::*;

    #[test]
    fn get_attribute_returns_the_value_of_the_attribute_as_set() {
        let mut subject = Player::new();

        subject
            .attributes
            .add(SkillTime(Fishing.into()), (-3, 0, Level(Fishing.into())));
        subject
            .attributes
            .add(SkillTime(Fishing.into()), (-2, 30000, Effect));

        assert_eq!(subject.get_attribute(SkillTime(Fishing.into()), 0), -5);
    }

    #[test]
    fn add_attributes_adds_an_attribute() {
        let mut subject = Player::new();

        subject.add_buff(SkillTime(Fishing.into()), (-3, 0, Level(Fishing.into())));
        subject.add_buff(SkillTime(Fishing.into()), (-2, 30000, Effect));

        assert_eq!(subject.get_attribute(SkillTime(Fishing.into()), 0), -5);
    }

    #[test]
    fn remove_buff_removes_a_buff_with_a_particular_tag() {
        let mut subject = Player::new();

        subject.add_buff(SkillTime(Fishing.into()), (-3, 0, Level(Fishing.into())));
        subject.add_buff(SkillTime(Fishing.into()), (-2, 30000, Effect));

        subject.remove_buff(Effect);

        assert_eq!(subject.get_attribute(SkillTime(Fishing.into()), 0), -3);
    }
}
