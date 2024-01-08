use bevy::{
    prelude::*,
    window::{WindowResolution, PrimaryWindow},
    math::Vec3Swizzles,
};
use bevy_rapier2d::prelude::*;

use crate::{
    object::ObjectBundle,
    PlayerAimingEvent,
    SCALE_FACTOR,
};

#[derive(Bundle, Default)]
pub struct GunBundle {
    pub object_bundle: ObjectBundle,
    pub gun: Gun,
}

#[derive(Component, Default)]
pub struct Gun {
    //damage: f32,
    //round_mass: f32,
}

pub fn shoot(
    buttons: Res<Input<MouseButton>>,
    mut commands: Commands,
    gun: Query<(&Transform, With<Gun>, With<InHand>)>, //look for guns held in the player's hands 
    mut ev_playeraiming: EventReader<PlayerAimingEvent>, //if the player is aiming
) {
    for ev in ev_playeraiming.read() {
        if ev.0 == true {
            for (trans, _, _) in &gun {
                if buttons.just_pressed(MouseButton::Left) {
                    let (bx, by) = (0.00965*SCALE_FACTOR, 0.0105*SCALE_FACTOR);//9mm bullet dimensions
                    let bullet = commands.spawn(BulletBundle{
                        obj_bundle: ObjectBundle::new(
                            Vec2::new(trans.translation.x+bx, trans.translation.y+by),
                            Vec2::new(bx, by),
                            Some(11340./SCALE_FACTOR)//density of lead in kg/m3
                        ),
                        bullet: Bullet,
                    }).id();
                    
                    let gun_forward = (trans.rotation * Vec3::Y).xy() * 20.;
                    //println!("{:?}", gun_forward);
                    commands.entity(bullet).insert(ExternalImpulse{
                        impulse: gun_forward, //this is fucked
                        torque_impulse: 0.,
                    });
                }
            }
        }
    }
}

pub fn gun_aiming(
    mut gun_data: Query<(&mut Gun, &mut ExternalImpulse, &Transform, With<RigidBody>, With<InHand>)>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut ev_playeraiming: EventReader<PlayerAimingEvent>, //if the player is aiming
) {
    for ev in ev_playeraiming.read() {
        if ev.0 == true {
            for (mut gun, mut ext_impulse, gun_trans, _, _) in &mut gun_data {
                let (camera, camera_transform) = camera_q.single();

                //copypaste from player aiming, all this should be a function that gets shared between these systems
                if let Some(mouse_world_position) = windows.single().cursor_position()
                    .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                    .map(|ray| ray.origin.truncate()) 
                {
                    //https://github.com/bevyengine/bevy/blob/main/examples/2d/rotation.rs for reference on the following code
                    let gun_pos = gun_trans.translation.xy();

                    let gun_forward = (gun_trans.rotation * Vec3::Y).xy();

                    //vector from player to mouse
                    let to_mouse = (mouse_world_position - gun_pos).normalize();

                    //get dot product between player forward vector and direction to the mouse
                    let forward_dot_mouse = gun_forward.dot(to_mouse);

                    //if player is already facing mouse
                    if (forward_dot_mouse - 1.0).abs() < f32::EPSILON {
                        continue;
                    }

                    //get right vector of player
                    let gun_right = (gun_trans.rotation * Vec3::X).xy();

                    //if negative, rotate CCW, if positive rotate CW
                    let right_dot_mouse = gun_right.dot(to_mouse);

                    let rotation_sign = -f32::copysign(0.0005, right_dot_mouse);

                    ext_impulse.torque_impulse = rotation_sign;
                    
                }
            
            }
        }
    }
}
