use crate::reagents::*;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component, Debug, Default)]
pub struct Liver {
    holding: Vec<Reagent>,
    health: u8,
}