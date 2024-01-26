use bevy::prelude::*;
use bevy_egui::egui::Key;
use bevy_rapier2d::prelude::*;

use crate::{
    body::hands::{GiveItem, Hands},
    body::legs::Legs,
    body::organs::{liver::Liver, stomach::Stomach, Organs},
    body::*,
    gun::{Gun, GunBundle},
    InHand, Item, ObjectBundle, PrimaryWindow, SCALE_FACTOR,
};

// This should be turned into a bundle
#[derive(Component, Default)]
pub struct Player {
    inventory: Vec<Item>,
}

pub fn spawn_player(
    mut commands: Commands,
    mut rapier_config: ResMut<RapierConfiguration>,
    mut hand_events: EventWriter<GiveItem>,
) {
    // Set gravity to 0.0 and spawn camera.
    rapier_config.gravity = Vec2::ZERO;
    commands.spawn(Camera2dBundle::default());

    let player_size = 0.8 * SCALE_FACTOR;

    let organs = Organs::default(&mut commands);
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
            Hands::human_hands(),     //i got hands! wow!
            Legs::human_flesh_legs(), //wowee, legs!
            //Liver::default(),
            //Stomach::default(),
            organs,
            Name::new("Player"),
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
            Name::new("Gun, 9mm"),
        ))
        .id(); //spawn a gun with inhand component
               //let name = names.get(gun).unwrap();
               //println!("{}", name);
    hand_events.send(GiveItem {
        receiver: Some(player),
        item: gun,
    });

    let joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(20.0, 55.0))
        .local_anchor2(Vec2::new(0.0, -35. / 2.));
    commands
        .entity(gun)
        .insert(ImpulseJoint::new(player, joint));
    //manually join the gun to the player (in the future this should be done with a pickup/inv system)
}

pub fn player_controls(
    keyboard_input: Res<Input<KeyCode>>,
    mut ev_playeraiming: EventWriter<PlayerAimingEvent>,
    mut ev_playerpoint: EventWriter<PlayerPointEvent>,
    mut ev_movement: EventWriter<MovementEvent>,
    mut player_data: Query<(With<Player>, &Legs)>,
    buttons: Res<Input<MouseButton>>,
    player: Query<Entity, With<Player>>,
) {
    let player_entity = player.get_single().unwrap();
    for (_, legs) in &player_data {
        //default movetype is Run, if right mb pressed, set movetype to Walk
        let mut movetype = MoveType::Run;
        if buttons.pressed(MouseButton::Right) {
            movetype = MoveType::Walk;
            ev_playeraiming.send(PlayerAimingEvent(true));
        }

        if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
            ev_movement.send(MovementEvent {
                target: player_entity,
                dir: MoveDir::Up,
                kind: movetype,
            })
        }
        if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
            ev_movement.send(MovementEvent {
                target: player_entity,
                dir: MoveDir::Left,
                kind: movetype,
            })
        }
        if keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]) {
            ev_movement.send(MovementEvent {
                target: player_entity,
                dir: MoveDir::Right,
                kind: movetype,
            })
        }
        if keyboard_input.any_pressed([KeyCode::S, KeyCode::Down]) {
            ev_movement.send(MovementEvent {
                target: player_entity,
                dir: MoveDir::Down,
                kind: movetype,
            })
        }
    }
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
        &Legs,
        &mut ExternalImpulse,
        &Transform,
        With<RigidBody>,
    )>,
    mut ev_playeraiming: EventReader<PlayerAimingEvent>,
    mut ev_playerpoint: EventWriter<PlayerPointEvent>,
    //time_step: Res<FixedTime>,
) {
    for (_, legs, mut ext_impulse, player_trans, _) in &mut player_data {
        //TODO: make player travel faster if they're moving in the direction they're pointing
        //TODO: if not aiming, movement keys should rotate player in direction of travel

        let linear_motility = legs.get_walk();
        let angular_motility = legs.get_agility();
        let aiming_motility = legs.get_aiming_speed();
        //if player not aiming, this might be fucked
        if ev_playeraiming.is_empty() {
            if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
                let point_spot = Vec2 {
                    x: player_trans.translation.x,
                    y: player_trans.translation.y + 10.,
                };
                ev_playerpoint.send(PlayerPointEvent {
                    point: point_spot,
                    speed: angular_motility,
                });
                ext_impulse.impulse.y += linear_motility;
            }
            if keyboard_input.any_pressed([KeyCode::R, KeyCode::Down]) {
                let point_spot = Vec2 {
                    x: player_trans.translation.x,
                    y: player_trans.translation.y - 10.,
                };
                ev_playerpoint.send(PlayerPointEvent {
                    point: point_spot,
                    speed: angular_motility,
                });
                ext_impulse.impulse.y -= linear_motility;
            }
            if keyboard_input.any_pressed([KeyCode::S, KeyCode::Right]) {
                let point_spot = Vec2 {
                    x: player_trans.translation.x + 10.0,
                    y: player_trans.translation.y,
                };
                ev_playerpoint.send(PlayerPointEvent {
                    point: point_spot,
                    speed: angular_motility,
                });
                ext_impulse.impulse.x += linear_motility;
            }
            if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
                let point_spot = Vec2 {
                    x: player_trans.translation.x - 10.0,
                    y: player_trans.translation.y,
                };
                ev_playerpoint.send(PlayerPointEvent {
                    point: point_spot,
                    speed: angular_motility,
                });
                ext_impulse.impulse.x -= linear_motility;
            }
        } else {
            if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
                ext_impulse.impulse.y += aiming_motility;
            }
            if keyboard_input.any_pressed([KeyCode::R, KeyCode::Down]) {
                ext_impulse.impulse.y -= aiming_motility;
            }
            if keyboard_input.any_pressed([KeyCode::S, KeyCode::Right]) {
                ext_impulse.impulse.x += aiming_motility;
            }
            if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
                ext_impulse.impulse.x -= aiming_motility;
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
    mut player_data: Query<(
        With<Player>,
        With<RigidBody>,
        &mut ExternalImpulse,
        &Transform,
        &Legs,
    )>,
    mut ev_playeraiming: EventReader<PlayerAimingEvent>,
    mut ev_playerpoint: EventWriter<PlayerPointEvent>,
    //time_step: Res<FixedTime>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    for (_, _, mut ext_impulse, player_trans, legs) in &mut player_data {
        for ev in ev_playeraiming.read() {
            let (camera, camera_transform) = camera_q.single();

            //horrific copypaste monstrosity please help i don't know how closures work
            if let Some(mouse_world_position) = windows
                .single()
                .cursor_position()
                .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
                .map(|ray| ray.origin.truncate())
            {
                ev_playerpoint.send(PlayerPointEvent {
                    point: mouse_world_position,
                    speed: legs.get_agility(),
                })
            }
        }
    }
}

#[derive(Event, Debug)]
pub struct PlayerPointEvent {
    pub point: Vec2,
    pub speed: f32,
}

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
            let to_mouse = (ev.point - player_pos).normalize();

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

            let rotation_sign = -f32::copysign(ev.speed, right_dot_mouse);

            ext_impulse.torque_impulse = rotation_sign;
        }
    }
}
