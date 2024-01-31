use crate::body::Organ;
use crate::helpers::*;
use crate::reagents::*;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component, Debug, Default, Reflect)]
pub struct Stomach {
    pub holding: Vec<Reagent>,
    health: u8,
}

impl Organ for Stomach {
    fn list_reagents(&self) -> HashMap<Reagent, u32> {
        count_element_function(self.holding.clone())
    }
    fn get_reagent(&self, target: Reagent) -> u32 {
        count(&self.holding, &&target)
    }
}
