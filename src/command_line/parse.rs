use once_cell::sync::Lazy;
use regex::Regex;

static FIRST_PASS: Lazy<Regex> = Lazy::new(|| Regex::new(r"\/([a-z]+)\s*((.|\n)*)").unwrap());

pub fn parse(command: &str) -> Option<(&str, &str)> {
    let caps = FIRST_PASS.captures(command);
    if let Some(caps) = caps {
        Some((caps.get(1).unwrap().into(), caps.get(2).unwrap().into()))
    } else {
        None
    }
}
