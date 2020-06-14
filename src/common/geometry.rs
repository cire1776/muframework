use regex::Regex;

pub struct Point {}

impl Point {
    pub fn read_coordinates(string: String) -> (i32, i32) {
        let re = Regex::new(r"\s*(\d+)\s*,\s*(\d+)").unwrap();
        let captures = re.captures(&string).unwrap();
        let x = captures
            .get(1)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .expect("unable to parse x");
        let y = captures
            .get(2)
            .unwrap()
            .as_str()
            .parse::<i32>()
            .expect("unable to parse y");

        (x, y)
    }
}
