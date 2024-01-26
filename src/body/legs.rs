use crate::agent::player::Player;
use bevy::prelude::*;

#[derive(Debug)]
pub struct Leg {
    pub lin_strength: f32,  //value representing run speed
    pub ang_strength: f32,  //value representing leg torque ability (affects turning speed)
    pub walk_modifier: f32, //lin_strength * ang_modifier = movement speed while walking
    pub health: u8,
}

impl Leg {
    pub fn default() -> Leg {
        Leg {
            //normie human legs
            lin_strength: 60.,
            ang_strength: 0.15,
            walk_modifier: 0.6,
            health: 100,
        }
    }
}

#[derive(Component, Debug, Default)]
pub struct Legs {
    pub legs: Vec<Leg>,
}

impl Legs {
    pub fn human_flesh_legs() -> Self {
        Self {
            legs: vec![Leg::default(), Leg::default()],
        }
    }
    pub fn default() -> Self {
        Self {
            legs: vec![Leg::default(), Leg::default()],
        }
    }
    //get the aggregate legginess of a grouping of legs for calculation of vector motility
    pub fn get_run_speed(&self) -> f32 {
        //wacky closure map thing!
        self.legs.iter().map(|i| i.lin_strength as f32).sum()
    }
    pub fn get_agility(&self) -> f32 {
        self.legs.iter().map(|i| i.ang_strength as f32).sum()
    }
    pub fn get_walk_speed(&self) -> f32 {
        self.legs
            .iter()
            .map(|i| i.lin_strength * i.walk_modifier as f32)
            .sum()
    }
}
