use crate::reagents::Reagent;
use bevy::prelude::*;

//reagent container component for item entities that can contain reagents
#[derive(Component, Reflect, Default)]
pub struct Container(pub Vec<Reagent>);
