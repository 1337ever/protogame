use crate::{
    body::{organs::*, Body},
    SCALE_FACTOR,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Default)]
pub struct NPC;

pub fn spawn_npc(mut commands: Commands) {
    let npc_size = 0.8 * SCALE_FACTOR;
    //theres prob a better way to do this but i cant bother. organs needs Commands bc it needs to spawn entities
    let organs = Organs::default(&mut commands);
    let npc = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(npc_size, npc_size)),
                    ..Default::default()
                },
                ..Default::default()
            },
            RigidBody::Dynamic,
            Velocity::zero(),
            Collider::cuboid(npc_size / 2.0, npc_size / 2.0),
            ColliderMassProperties::Density(985. / SCALE_FACTOR), //avg density of human body, dividing like this is prob wrong
            NPC::default(),
            ExternalImpulse::default(), //use impulses instead of velocity so controls are affected by mass
            Damping {
                linear_damping: 3.,
                angular_damping: 3.,
            },
            Body::default(),
            organs, //bodies and organs are separate, to allow for bodies without organs
            Name::new("NPC 1"),
        ))
        .id();
}
