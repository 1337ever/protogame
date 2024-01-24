use bevy::prelude::*;
pub mod legs;
pub mod hands;
pub mod organs;

use crate::body::{
    legs::*, 
    hands::*,
    organs::*,
};

#[derive(Bundle)]
pub struct Body {
    legs: Legs,
    hands: Hands,
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug)]
pub enum MoveType {
    Run,
    Walk,
    Crawl,
}

#[derive(Event, Debug)]
pub struct MovementEvent {
    dir: Direction,
    kind: MoveType,
}


