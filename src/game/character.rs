use super::*;
use regex::Regex;

pub struct Player {
    pub id: u64,
    pub x: i32,
    pub y: i32,
    pub facing: Direction,
    pub character_type: CharacterType,
    pub mounting_points: MountingPointMap,
    pub external_inventory: Option<Vec<Item>>,
    pub endorsements: HashMap<String, bool>,
    pub activity_guard: Option<Guard>,
    pub activity_timer: Option<timer::Timer>,
}

impl Player {
    pub fn new() -> Player {
        Player {
            id: 1,
            x: 0,
            y: 0,
            facing: Direction::Up,
            character_type: CharacterType::Player,
            mounting_points: MountingPointMap::new(),
            external_inventory: None,
            endorsements: HashMap::new(),
            activity_guard: None,
            activity_timer: None,
        }
    }

    pub fn inventory_id(&self) -> u64 {
        self.id
    }

    pub fn is_endorsed_with<S: ToString>(&self, endorsement: S) -> bool {
        self.endorsements.contains_key(&endorsement.to_string())
    }

    pub fn endorse_with<S: ToString>(&mut self, endorsement: S) {
        self.endorsements.insert(endorsement.to_string(), true);
    }

    pub fn unendorse_with<S: ToString>(&mut self, endorsement: S) {
        self.endorsements.remove(&endorsement.to_string());
    }
}

pub enum CharacterFacing {
    Up,
    Upright,
    Right,
    Downright,
    Down,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Clone)]
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
