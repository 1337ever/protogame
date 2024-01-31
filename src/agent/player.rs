use bevy::prelude::*;
use bevy_egui::egui::Key;
use bevy_rapier2d::prelude::*;

use crate::{
    body::hands::{GiveItem, Hands},
    body::legs::Legs,
    body::organs::{liver::Liver, stomach::Stomach, Organs},
    body::*,
    gun::{Gun, GunBundle},
    InHand, ObjectBundle, PrimaryWindow, SCALE_FACTOR,
};

use self::head::Mouth;
use self::head::Head;

// This should be turned into a bundle
#[derive(Component, Default)]
pub struct Player {
    inventory: Vec<Entity>,
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

    //theres prob a better way to do this but i cant bother. organs needs Commands bc it needs to spawn entities
    let organs = Organs::default(&mut commands);
    let mouth = Mouth::with_xuyin(&mut commands);
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
            Body {
                head: Head {mouth: mouth},
                ..Default::default()
            },
            organs, //bodies and organs are separate, to allow for bodies without organs
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
    mut ev_movement: EventWriter<MovementEvent>,
    buttons: Res<Input<MouseButton>>,
    player: Query<Entity, With<Player>>,
) {
    let player_entity = player.get_single().unwrap();
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

#[derive(Event, Debug)]
pub struct PlayerAimingEvent(pub bool);

pub fn player_aiming(
    player: Query<Entity, With<Player>>,
    mut ev_playeraiming: EventReader<PlayerAimingEvent>,
    mut ev_playerpoint: EventWriter<PointEvent>,
    //time_step: Res<FixedTime>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    for ev in ev_playeraiming.read() {
        let (camera, camera_transform) = camera_q.single();

        //horrific copypaste monstrosity to find the position of the mouse
        if let Some(mouse_world_position) = windows
            .single()
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            ev_playerpoint.send(PointEvent {
                target: player.get_single().unwrap(),
                point: mouse_world_position,
            })
        }
    }
}
