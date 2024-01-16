use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Default)]
pub struct Object;
#[derive(Bundle)]
pub struct ObjectBundle {
    //bundle for physical objects in the world (most things)
    marker: Object,
    body: RigidBody,
    velocity: Velocity,
    collider: Collider,
    mass_properties: ColliderMassProperties,
    ext_impulse: ExternalImpulse,
    damping: Damping,
    sprite_bundle: SpriteBundle,
}
impl Default for ObjectBundle {
    fn default() -> Self {
        let (size_x, size_y) = (10.0, 10.0);
        Self {
            marker: Object,
            body: RigidBody::Dynamic,
            velocity: Velocity::zero(),
            collider: Collider::cuboid(size_x / 2., size_y / 2.),
            mass_properties: ColliderMassProperties::Density(1.0),
            ext_impulse: ExternalImpulse::default(),
            damping: Damping {
                linear_damping: 1.,
                angular_damping: 1.,
            },
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(size_x, size_y)),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}
impl ObjectBundle {
    pub fn new(pos: Vec2, size: Vec2, density: Option<f32>) -> Self {
        //optionally specify the density of the object
        let mass_prop = match density {
            None => ColliderMassProperties::Density(1.0),
            Some(density) => ColliderMassProperties::Density(density),
        };

        ObjectBundle {
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(size),
                    ..Default::default()
                },
                transform: Transform::from_xyz(pos.x, pos.y, 0.),
                ..default()
            },
            collider: Collider::cuboid(size.x / 2., size.y / 2.),
            mass_properties: mass_prop,
            ..default()
        }
    }
}
