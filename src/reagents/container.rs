use crate::reagents::Reagent;
use bevy::prelude::*;

//reagent container component for item entities that can contain reagents
#[derive(Component, Reflect)]
pub struct Container {
    pub holding: Vec<Reagent>,
}
