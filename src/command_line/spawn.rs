use bevy::prelude::*;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::physics::hitbox::SpecializedAabb;

static GET_JSON: Lazy<Regex> = Lazy::new(|| Regex::new(r"([A-z]+)\s+((.|\n)*)").unwrap());

macro_rules! supports {
    ($bundle:expr, $commands:expr, $json:expr, [$($Type:ty),*]) => {
        match $bundle {
            $(
                stringify!($Type) => {
                    match serde_json::from_str::<$Type>($json) {
                        Ok(value) => { $commands.spawn(value); }
                        Err(e) => println!("{:?}", e),
                    };
                }
            )*
            _ => println!("cannot spawn bundle of type {}", &$bundle),
        }
    };
}

pub fn spawn(commands: &mut Commands, command_str: &str) {
    if let Some(caps) = GET_JSON.captures(command_str) {
        let bundle: &str = caps.get(1).unwrap().into();
        let json: &str = caps.get(2).unwrap().into();

        supports!(bundle, commands, json, [SpecializedAabb]);
    }
}
