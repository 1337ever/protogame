use crate::reagents::*;
use bevy::prelude::*;

pub mod cigarette;
pub mod food;

//might implement a smell system, but that might be going a little too far

#[derive(Component)]
pub struct Item {
    pub name: Name, //can i have components inside components? let's see
    pub desc: String,
}

impl Item {
    pub fn default() -> Self {
        Item {
            name: Name::new("Cow Sphere"),
            desc: "Strange. This shouldn't be here.".to_string(),
        }
    }
}
