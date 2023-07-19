use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Bundle)]
pub struct Ball {
    collider: Collider,
    transform_bundle: TransformBundle,
    rigid_body: RigidBody,
}

impl<'de> serde::Deserialize<'de> for Ball {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(serde::Deserialize)]
        struct S {
            position: Vec2,
            radius: f32,
        }

        impl S {
            fn into_ball_bundle(&self) -> Ball {
                Ball {
                    collider: Collider::ball(self.radius),
                    transform_bundle: TransformBundle::from_transform(Transform::from_xyz(
                        self.position.x,
                        self.position.y,
                        0.,
                    )),
                    rigid_body: RigidBody::Dynamic,
                }
            }
        }

        S::deserialize(deserializer).map(|s| s.into_ball_bundle())
    }
}

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

#[allow(unused)]
pub fn spawn(commands: &mut Commands, command_str: &str) {
    if let Some(caps) = GET_JSON.captures(command_str) {
        let bundle: &str = caps.get(1).unwrap().into();
        let json: &str = caps.get(2).unwrap().into();

        supports!(bundle, commands, json, [Ball]);
    }
}
