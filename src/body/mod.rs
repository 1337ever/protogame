use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub mod hands;
pub mod legs;
pub mod organs;

use crate::body::{hands::*, legs::*, organs::*};

//Module for things that have bodies (players, AI); so they can all use the same movement code

#[derive(Bundle, Debug, Default)]
pub struct Body {
    legs: Legs,
    hands: Hands,
}

#[derive(Debug)]
pub enum MoveDir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
pub enum MoveType {
    Run,
    Walk,
    Crawl,
}

#[derive(Event, Debug)]
pub struct PointEvent {
    pub target: Entity, //entity that we want to point at some location
    pub point: Vec2,    //point that we want to point the entity at
}

pub fn handle_point_body(
    //vvvv Takes in coordinate to point to via event
    mut ev_point: EventReader<PointEvent>,
    mut body_data: Query<(&mut ExternalImpulse, &Transform, &Legs, With<RigidBody>)>,
) {
    for ev in ev_point.read() {
        if body_data.contains(ev.target) {
            if let Ok(target) = body_data.get_mut(ev.target) {
                let (mut ext_impulse, trans, legs) = (target.0, target.1, target.2);
                let pos = trans.translation.xy();

                let forward = (trans.rotation * Vec3::Y).xy();

                //vector from body to point
                let to_point = (ev.point - pos).normalize();

                //get dot product between body forward vector and direction to the point
                let forward_dot_point = forward.dot(to_point);

                //if player is already facing mouse
                if (forward_dot_point - 1.0).abs() < f32::EPSILON {
                    continue;
                }

                //get right vector of body
                let right = (trans.rotation * Vec3::X).xy();

                //if negative, rotate CCW, if positive rotate CW
                let right_dot_point = right.dot(to_point);

                let rotation_sign = -f32::copysign(legs.get_agility(), right_dot_point);

                ext_impulse.torque_impulse = rotation_sign;
            }
        }
    }
}

#[derive(Event, Debug)]
pub struct MovementEvent {
    pub target: Entity,
    pub dir: MoveDir,
    pub kind: MoveType,
}

pub fn handle_movement_event(
    mut commands: Commands,
    mut move_ev: EventReader<MovementEvent>,
    mut phys_data: Query<(With<RigidBody>, &Legs, &mut ExternalImpulse, &Transform)>,
) {
    for (_, legs, ext_impulse, transform) in &phys_data {
        for ev in move_ev.read() {
            if phys_data.contains(ev.target) {
                //println!("success");
            }
        }
    }
}
