use crate::{body::liver::*, body::stomach::*, reagents::*};
use bevy::prelude::*;
use std::collections::HashMap;

use self::brain::BrainBundle;
pub mod brain;
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

#[derive(Component, Debug, Reflect)]
pub struct Organs {
    //smth abt this weirds me out but whatever it's the ECS Way
    pub organs: Vec<Entity>,
}

impl Organs {
    pub fn default(commands: &mut Commands) -> Organs {
        Organs {
            organs: vec![
                commands
                    .spawn((Stomach::default(), Name::new("Stomach")))
                    .id(),
                commands.spawn((Liver::default(), Name::new("Liver"))).id(),
                commands
                    .spawn((BrainBundle::default(), Name::new("Brain")))
                    .id(),
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

pub fn reagents_metabolisis(
    mut stomach_query: Query<&mut Stomach>,
    mut liver_query: Query<&mut Liver>,
) {
}
