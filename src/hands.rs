use bevy::prelude::*;

#[derive(Component, Debug, Default, Clone)]
pub struct Hands {
    pub hands: Vec<Hand>,
    pub active: Option<usize>,
}

#[derive(Debug, Default, Clone)]
pub struct Hand {
    pub holding: Option<Entity>,
}