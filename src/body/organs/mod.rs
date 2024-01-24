use bevy::prelude::*;
use std::collections::HashMap;
use crate::reagents::*;
pub mod liver;
pub mod stomach;

pub trait Organ {
    //list all reagents and the number of times they appear (their quantity)
    fn list_reagents(&self) -> HashMap<Reagent, u32>;
    //get quantity of a single type of reagent
    fn get_reagent(&self, target: Reagent) -> u32;
    fn process_reagents(&self);
}

#[derive(Component)]
pub struct Organs<T: Organ> {
    organs: Vec<T>,
}

/* 
prob not using this stuff, just make organs components
will come back to bite me when dealing with organ implantation and removal
pub struct Organ<T> {
    organ: T,
    health: u8,
}
*/