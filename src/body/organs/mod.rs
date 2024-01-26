use crate::{body::liver::*, body::stomach::*, reagents::*};
use bevy::prelude::*;
use std::collections::HashMap;
pub mod liver;
pub mod stomach;

pub trait Organ {
    //list all reagents and the number of times they appear (their quantity)
    fn list_reagents(&self) -> HashMap<Reagent, u32>;
    //get quantity of a single type of reagent
    fn get_reagent(&self, target: Reagent) -> u32;
    //fn process_reagents(&self);
}

//TODO: add a GiveReagent system like the hands system
#[derive(Event)]
pub struct GiveReagent {
    pub receiver: Entity,
    pub reagent: Reagent,
}

#[derive(Component, Debug)]
pub struct Organs {
    //smth abt this weirds me out but whatever it's the ECS Way
    pub organs: Vec<Entity>,
}

impl Organs {
    pub fn default(commands: &mut Commands) -> Organs {
        Organs {
            //organs: vec![Stomach::default(), Liver::default()]
            //organs: Vec::<T>::from([Stomach::default(), Liver::default()])
            organs: vec![
                commands
                    .spawn((Stomach::default(), Name::new("Stomach")))
                    .id(),
                commands.spawn((Liver::default(), Name::new("Liver"))).id(),
            ],
        }
    }
}

pub fn handle_give_reagent(
    mut commands: Commands,
    mut events: EventReader<GiveReagent>,
    mut organs: Query<&mut Organs>,
    //player: Query<Entity, With<Player>>,
    items: Query<&Name>,
) {
    for ev in events.read() {}
}

pub fn test_reagents_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut stomach_query: Query<&mut Stomach>,
) {
    if keyboard_input.just_pressed(KeyCode::P) {
        for mut stomach in &mut stomach_query {
            stomach.holding.push(Reagent::Water);
        }
    }
    if keyboard_input.just_pressed(KeyCode::L) {
        for mut stomach in &mut stomach_query {
            stomach.holding.push(Reagent::Toxin);
        }
    }
}

pub fn process_reagents_system(
    mut stomach_query: Query<&mut Stomach>,
    mut liver_query: Query<&mut Liver>,
) {
}
/*
prob not using this stuff, just make organs components
will come back to bite me when dealing with organ implantation and removal
pub struct Organ<T> {
    organ: T,
    health: u8,
}
*/
