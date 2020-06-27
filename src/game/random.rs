use super::*;
use extern_rand::prelude::*;
use extern_rand::Rng as extern_Rng;

pub struct Rng {
    tags: HashMap<&'static str, i128>,
    test_mode: bool,
}

impl Rng {
    pub fn new() -> Self {
        Self {
            tags: HashMap::new(),
            test_mode: false,
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
                self.check_for_test_mode(tag);
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
                self.check_for_test_mode(tag);
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
                self.check_for_test_mode(tag);
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

    pub fn set_test_mode(&mut self) {
        self.test_mode = true;
    }

    fn check_for_test_mode(&self, tag: &'static str) {
        if self.test_mode {
            panic!("tag not set in test mode: {}", tag)
        }
    }
}
