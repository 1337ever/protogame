use bevy::prelude::*;
use clap::ValueEnum;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, ValueEnum)]
pub enum Reagent {
    Water,
    Toxin,
}
