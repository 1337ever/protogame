use crate::reagents::*;
use bevy::prelude::*;
use counter::Counter;
use std::collections::HashMap;
use crate::body::Organ;

#[derive(Component, Debug, Default)]
pub struct Stomach {
    holding: Vec<Reagent>,
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

//copypasta
fn count_element_function<I>(it: I) -> HashMap<I::Item, u32>
where
    I: IntoIterator,
    I::Item: Eq + core::hash::Hash,
{
    let mut result = HashMap::new();

    for item in it {
        *result.entry(item).or_insert(0) += 1;
    }

    result
}

fn count<I>(it: I, item: &I::Item) -> u32
where
    I: IntoIterator,
    I::Item: PartialEq,
{
    it.into_iter().filter(|x| x == item).count() as u32
}


impl Organ for Stomach {
    fn list_reagents(&self) -> HashMap<Reagent, u32> {
        //let reagents_list = HashMap::new();

        
        //reagents_list
        count_element_function(self.holding.clone())
    }
    fn get_reagent(&self, target: Reagent) -> u32 {
        count(&self.holding, &&target)
    }
    fn process_reagents(&self) {
        
    }
}