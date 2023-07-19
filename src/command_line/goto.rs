use super::*;
use once_cell::sync::Lazy;
use regex::Regex;

static GET_COORDS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\(\s*(-?(\d|\.)+),\s*(-?(\d|\.)+),?\s*\)").unwrap());

pub(crate) fn goto(q_player: &mut Query<&mut Transform, With<Player>>, body: &str) {
    if let Ok(mut transform) = q_player.get_single_mut() {
        if let Some((x, y)) = {
            if let Some(captures) = GET_COORDS.captures(body) {
                Some((captures.get(1).unwrap(), captures.get(3).unwrap()))
            } else {
                None
            }
        } {
            transform.translation.x = str::parse(x.into()).unwrap();
            transform.translation.y = str::parse(y.into()).unwrap();
            println!("set transaltion to {:?}", transform.translation);
        }
    } else {
        println!("could not get single_mut from player query");
    }
}
