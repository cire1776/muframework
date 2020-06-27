use super::*;
use extern_rand::prelude::*;
use extern_rand::Rng as extern_Rng;

pub struct Rng {
    tags: HashMap<&'static str, i128>,
}

impl Rng {
    pub fn new() -> Self {
        Self {
            tags: HashMap::new(),
        }
    }

    pub fn percentile(&mut self, chance: u8, tag: &'static str) -> bool {
        let tag_value = self.tags.get(tag);
        let value = match tag_value {
            Some(value) => {
                if *value >= 100 {
                    panic!("invalid tag setting");
                }
                *value
            }
            None => {
                let mut rng = thread_rng();
                rng.gen_range(0, 100) as i128
            }
        };
        value < chance as i128
    }

    pub fn range(&mut self, low: i128, high: i128, tag: &'static str) -> i128 {
        let tag_value = self.tags.get(tag);
        match tag_value {
            Some(value) => *value,
            None => {
                let mut rng = thread_rng();
                rng.gen_range(low, high)
            }
        }
    }

    pub fn succeeds(&mut self, low: i128, high: i128, tag: &'static str) -> bool {
        let tag_value = self.tags.get(tag);
        let roll = match tag_value {
            Some(value) => *value,
            None => {
                let mut rng = thread_rng();
                rng.gen_range(low, high)
            }
        };
        roll == 0
    }

    pub fn set(&mut self, tag: &'static str, value: i128) {
        self.tags.insert(tag, value);
    }

    pub fn clear(&mut self) {
        self.tags.clear();
    }
}
