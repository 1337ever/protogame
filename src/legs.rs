use crate::player::Player;
use bevy::prelude::*;

pub struct Leg {
    pub lin_strength: f32, //value representing linear legginess
    pub ang_strength: f32, //value representing leg torque ability (affects turning speed)
    pub ang_modifier: f32, //lin_strength * ang_modifier = movement speed while aiming

}

#[derive(Component)]
pub struct Legs {
    pub legs: Vec<Leg>,
}

impl Legs {
    pub fn human_flesh_legs() -> Self {
        Self {
            legs: vec![Leg{lin_strength: 60.0, ang_strength: 0.15, ang_modifier: 0.60}, Leg{lin_strength: 60.0, ang_strength: 0.15, ang_modifier: 0.60}],
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
    pub fn get_aiming_speed(&self) -> f32 {
        self.legs.iter().map(|i| i.lin_strength*i.ang_modifier as f32).sum()
    }
}