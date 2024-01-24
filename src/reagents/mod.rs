use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Reagent {
    Water,
    Toxin,
}