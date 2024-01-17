use bevy::prelude::*;
use bevy_inspector_egui::egui::PlatformOutput;
use bevy_rapier2d::prelude::*;

use crate::{
    gun::{Gun, GunBundle},
    hands::Hands,
    InHand, Item, ObjectBundle, PrimaryWindow, World, SCALE_FACTOR,
};

// This should be turned into a bundle
#[derive(Component, Default)]
pub struct Player {
    inventory: Vec<Item>,
}

pub fn spawn_player(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    // Set gravity to 0.0 and spawn camera.
    rapier_config.gravity = Vec2::ZERO;
    commands.spawn(Camera2dBundle::default());

    let player_size = 0.8 * SCALE_FACTOR;

    // Spawn entity with `Player` struct as a component for access in movement query.
    let player = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(player_size, player_size)),
                    ..Default::default()
                },
                ..Default::default()
            },
            RigidBody::Dynamic,
            Velocity::zero(),
            Collider::cuboid(player_size / 2.0, player_size / 2.0),
            ColliderMassProperties::Density(985. / SCALE_FACTOR), //avg density of human body, dividing like this is prob wrong
            Player::default(),
            ExternalImpulse::default(), //use impulses instead of velocity so controls are affected by mass
            Damping {
                linear_damping: 3.,
                angular_damping: 3.,
            },
            Hands::human_hands(), //i got hands! wow!
        ))
        .id();

    let gun = commands
        .spawn((
            GunBundle {
                object_bundle: ObjectBundle::new(
                    Vec2::new(0., 200.),
                    Vec2::new(0.028 * SCALE_FACTOR * 2., 0.185 * SCALE_FACTOR),
                    None,
                ),
                gun: Gun {},
            },
            InHand,
        ))
        .id(); //spawn a gun with inhand component

    let joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(20.0, 55.0))
        .local_anchor2(Vec2::new(0.0, -35. / 2.));
    commands
        .entity(gun)
        .insert(ImpulseJoint::new(player, joint));
    //manually join the gun to the player (in the future this should be done with a pickup/inv system)
}


//TODO: make this a player controls system that also handles mouse inputs
//send events for both keyboard movement and mouse aiming, to cut down on number of systems
//trying to access the same stuff at the same time.
//also stop hardcoding so many things
pub fn player_movement(
    //monstrosity
    keyboard_input: Res<Input<KeyCode>>,
    mut player_data: Query<(
        With<Player>,
        &mut ExternalImpulse,
        &Transform,
        With<RigidBody>,
    )>,
    mut ev_playeraiming: EventReader<PlayerAimingEvent>,
    mut ev_playerpoint: EventWriter<PlayerPointEvent>,
    //time_step: Res<FixedTime>,
) {
    for (_, mut ext_impulse, player_trans, _) in &mut player_data {
        //TODO: make player travel faster if they're moving in the direction they're pointing
        //TODO: if not aiming, movement keys should rotate player in direction of travel

        //if player not aiming, this might be fucked
        if ev_playeraiming.is_empty() {
            if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
                let point_spot = Vec2 {
                    x: player_trans.translation.x,
                    y: player_trans.translation.y + 10.,
                };
                ev_playerpoint.send(PlayerPointEvent(point_spot));
                ext_impulse.impulse.y += 100.0;
            }
            if keyboard_input.any_pressed([KeyCode::R, KeyCode::Down]) {
                let point_spot = Vec2 {
                    x: player_trans.translation.x,
                    y: player_trans.translation.y - 10.,
                };
                ev_playerpoint.send(PlayerPointEvent(point_spot));
                ext_impulse.impulse.y -= 100.0;
            }
            if keyboard_input.any_pressed([KeyCode::S, KeyCode::Right]) {
                let point_spot = Vec2 {
                    x: player_trans.translation.x + 10.0,
                    y: player_trans.translation.y,
                };
                ev_playerpoint.send(PlayerPointEvent(point_spot));
                ext_impulse.impulse.x += 100.0;
            }
            if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
                let point_spot = Vec2 {
                    x: player_trans.translation.x - 10.0,
                    y: player_trans.translation.y,
                };
                ev_playerpoint.send(PlayerPointEvent(point_spot));
                ext_impulse.impulse.x -= 100.0;
            }
        } else {
            if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
                ext_impulse.impulse.y += 100.0;
            }
            if keyboard_input.any_pressed([KeyCode::R, KeyCode::Down]) {
                ext_impulse.impulse.y -= 100.0;
            }
            if keyboard_input.any_pressed([KeyCode::S, KeyCode::Right]) {
                ext_impulse.impulse.x += 100.0;
            }
            if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
                ext_impulse.impulse.x -= 100.0;
            }
            //no read() in this function, so event buffer must be manually cleared to
            //re-enable movement after aiming button released
            ev_playeraiming.clear();
        }
    }
}

#[derive(Event, Debug)]
pub struct PlayerAimingEvent(pub bool);

pub fn player_aiming(
    buttons: Res<Input<MouseButton>>,
    mut player_data: Query<(
        With<Player>,
        &mut ExternalImpulse,
        &Transform,
        With<RigidBody>,
    )>,
    mut ev_playeraiming: EventWriter<PlayerAimingEvent>,
    //time_step: Res<FixedTime>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    for (_, mut ext_impulse, player_trans, _) in &mut player_data {
        if buttons.pressed(MouseButton::Right) {
            ev_playeraiming.send(PlayerAimingEvent(true));

            //surely this .single will never have to be changed
            let (camera, camera_transform) = camera_q.single();

            //horrific copypaste monstrosity please help i don't know how closures work
            if let Some(mouse_world_position) = windows
                .single()
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                .map(|ray| ray.origin.truncate())
            {
                //https://github.com/bevyengine/bevy/blob/main/examples/2d/rotation.rs for reference on the following code
                let player_pos = player_trans.translation.xy();

                let player_forward = (player_trans.rotation * Vec3::Y).xy();

                //vector from player to mouse
                let to_mouse = (mouse_world_position - player_pos).normalize();

                //get dot product between player forward vector and direction to the mouse
                let forward_dot_mouse = player_forward.dot(to_mouse);

                //if player is already facing mouse
                if (forward_dot_mouse - 1.0).abs() < f32::EPSILON {
                    continue;
                }

                //get right vector of player
                let player_right = (player_trans.rotation * Vec3::X).xy();

                //if negative, rotate CCW, if positive rotate CW
                let right_dot_mouse = player_right.dot(to_mouse);

                let rotation_sign = -f32::copysign(0.1, right_dot_mouse);

                ext_impulse.torque_impulse = rotation_sign;
            }
        }
        if buttons.just_released(MouseButton::Right) {
            ev_playeraiming.send(PlayerAimingEvent(false))
        }
    }
}

#[derive(Event, Debug)]
pub struct PlayerPointEvent(pub Vec2);

//generalized system to point the player at some position
pub fn point_player(
    //vvvv Takes in coordinate to point to via event
    mut ev_playerpoint: EventReader<PlayerPointEvent>,
    mut player_data: Query<(
        With<Player>,
        &mut ExternalImpulse,
        &Transform,
        With<RigidBody>,
    )>,
) {
    for (_, mut ext_impulse, player_trans, _) in &mut player_data {
        for ev in ev_playerpoint.read() {
            //https://github.com/bevyengine/bevy/blob/main/examples/2d/rotation.rs for reference on the following code
            let player_pos = player_trans.translation.xy();

            let player_forward = (player_trans.rotation * Vec3::Y).xy();

            //vector from player to mouse
            let to_mouse = (ev.0 - player_pos).normalize();

            //get dot product between player forward vector and direction to the mouse
            let forward_dot_mouse = player_forward.dot(to_mouse);

            //if player is already facing mouse
            if (forward_dot_mouse - 1.0).abs() < f32::EPSILON {
                continue;
            }

            //get right vector of player
            let player_right = (player_trans.rotation * Vec3::X).xy();

            //if negative, rotate CCW, if positive rotate CW
            let right_dot_mouse = player_right.dot(to_mouse);

            let rotation_sign = -f32::copysign(0.1, right_dot_mouse);

            ext_impulse.torque_impulse = rotation_sign;
        }
    }
}
