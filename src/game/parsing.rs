use std::str::FromStr;

pub fn capture_coordinate(captures: &regex::Captures, index: usize) -> i32 {
    captures
        .get(index)
        .unwrap()
        .clone()
        .as_str()
        .parse::<i32>()
        .unwrap()
}

pub fn capture_integer<U: FromStr>(captures: &regex::Captures, index: usize) -> U {
    captures
        .get(index)
        .unwrap()
        .clone()
        .as_str()
        .parse::<U>()
        .ok()
        .expect("must be convertible to U")
}

pub fn capture_symbol<'a>(captures: &'a regex::Captures, index: usize) -> char {
    captures
        .get(index)
        .expect("unable to find symbol")
        .as_str()
        .chars()
        .nth(0)
        .unwrap()
}

pub fn capture_string<'a>(captures: &'a regex::Captures, index: usize) -> &'a str {
    captures
        .get(index)
        .expect("unable to parse string")
        .as_str()
}

pub fn capture_optional_string<'a>(captures: &'a regex::Captures, index: usize) -> &'a str {
    match captures.get(index) {
        Some(re_match) => re_match.as_str(),
        None => "",
    }
}

pub fn capture_section(captures: &regex::Captures, index: usize) -> Vec<String> {
    captures
        .get(index)
        .unwrap()
        .as_str()
        .to_string()
        .lines()
        .map(|s| s.to_string())
        .collect()
}
