use bevy::prelude::*;
use clap::ValueEnum;

pub mod container;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, ValueEnum)]
pub enum Reagent {
    Water,
    Toxin,
    Bicaridine,
    Nicotine,
    Protein,
    Omega3,
    Carb,
    Sugar,
    Salt,
}
