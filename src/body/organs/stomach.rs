use crate::body::Organ;
use crate::helpers::*;
use crate::reagents::*;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component, Debug, Default)]
pub struct Stomach {
    pub holding: Vec<Reagent>,
    health: u8,
}

/*
impl Stomach {
    pub fn get_reagents_list(&self) -> HashMap<Reagent, u32> {
        let reagents_list = HashMap::new();
        for reagent in self.holding.iter() {
            let count = 0;

        }

        reagents_list
    }
}*/

impl Organ for Stomach {
    fn list_reagents(&self) -> HashMap<Reagent, u32> {
        //let reagents_list = HashMap::new();

        //reagents_list
        count_element_function(self.holding.clone())
    }
    fn get_reagent(&self, target: Reagent) -> u32 {
        count(&self.holding, &&target)
    }
    /*
    fn process_reagents(&self) {

    }
    */
}
