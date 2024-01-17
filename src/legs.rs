use crate::player::Player;
use bevy::prelude::*;

pub struct Leg {
    pub lin_strength: f32, //value representing linear legginess
    pub ang_strength: f32, //value representing leg torque ability/agility

}

#[derive(Component)]
pub struct Legs {
    pub legs: Vec<Leg>,
}

impl Legs {
    pub fn human_flesh_legs() -> Self {
        Self {
            legs: vec![Leg{lin_strength: 25.0, ang_strength: 25.0}, Leg{lin_strength: 25.0, ang_strength: 25.0}],
        }
    }
    //get the aggregate legginess of a grouping of legs for calculation of vector motility
    pub fn get_walk(&self) -> f32 {
        //wacky closure map thing!
        self.legs.iter().map(|i| i.lin_strength as f32).sum()
    }
    pub fn get_agility(&self) -> f32 {
        self.legs.iter().map(|i| i.ang_strength as f32).sum()
    }
}