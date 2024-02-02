use bevy::prelude::*;

use super::container::Container;
use crate::object::Health;

#[derive(Component, Debug, Reflect)]
pub struct Brain {
    perception: u8,
}

impl Default for Brain {
    fn default() -> Self {
        Brain {
            perception: 100,
        }
    }
}

#[derive(Bundle, Default)]
pub struct BrainBundle {
    brain: Brain,
    container: Container,
    health: Health,
}
