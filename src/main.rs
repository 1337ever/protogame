use std::f32::consts::{PI, FRAC_2_PI};

use bevy::{
    prelude::*,
    window::{WindowResolution, PrimaryWindow},
    math::Vec3Swizzles,
};
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod hands;

const SCALE_FACTOR: f32 = 50.;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(1000., 1000.),
                    title: "Protogame".to_string(),
                    ..default()
                }),
                ..default()
            }),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(SCALE_FACTOR),
            RapierDebugRenderPlugin::default(),
        ))
        .add_systems(Startup, (spawn_player))
        .add_systems(Update, (player_movement, player_aiming, gun_aiming, shoot))
        .add_event::<PlayerAimingEvent>()
        .add_plugins(WorldInspectorPlugin::new())
        //.insert_resource(FixedTime::new_from_secs(1.0 / 165.0))
        .run();
}

#[derive(Component, Default)]
struct Item;
#[derive(Bundle, Default)]
struct ItemBundle { //bundle for things that can be picked up
    obj_bundle: ObjectBundle,
}

#[derive(Component)]
struct InHand; //marker component for things in the player's hands

#[derive(Component, Default)]
struct Object;
#[derive(Bundle)]
struct ObjectBundle {//bundle for physical objects in the world (most things)
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
            collider: Collider::cuboid(size_x/2., size_y/2.),
            mass_properties: ColliderMassProperties::Density(1.0),
            ext_impulse: ExternalImpulse::default(),
            damping: Damping { linear_damping: 1., angular_damping: 1. },
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(size_x, size_y)),
                    ..Default::default()
                },
                ..Default::default()
            }
        }
    }
}
impl ObjectBundle {
    fn new(pos: Vec2, size: Vec2, density: Option<f32>) -> Self {

        //optionally specify the density of the object
        let mass_prop = match density {
            None => ColliderMassProperties::Density(1.0),
            Some(density) => ColliderMassProperties::Density(density)
        };

        ObjectBundle{
            sprite_bundle: SpriteBundle{
                sprite: Sprite {
                    custom_size: Some(size),
                    ..Default::default()
                },
                transform: Transform::from_xyz(pos.x, pos.y, 0.),
                ..default()
            },
            collider: Collider::cuboid(size.x/2., size.y/2.),
            mass_properties: mass_prop,
            ..default()
        }
    }
}

#[derive(Bundle, Default)]
struct GunBundle {
    object_bundle: ObjectBundle,
    gun: Gun,
}

#[derive(Component, Default)]
struct Gun {
    //damage: f32,
    //round_mass: f32,
}

fn shoot(
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

fn gun_aiming(
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

#[derive(Bundle, Default)]
struct BulletBundle {
    obj_bundle: ObjectBundle,
    bullet: Bullet,
}
//9mm bullet length: 10.54mm
//width: 9.65mm
//weight: 5.2g
#[derive(Component, Default)]
struct Bullet;

// This should be turned into a bundle
#[derive(Component, Default)]
struct Player {
    inventory: Vec<Item>,
}

fn spawn_player(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    // Set gravity to 0.0 and spawn camera.
    rapier_config.gravity = Vec2::ZERO;
    commands.spawn(Camera2dBundle::default());

    let player_size = 0.8 * SCALE_FACTOR;

    // Spawn entity with `Player` struct as a component for access in movement query.
    let player = commands.spawn((
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
        ColliderMassProperties::Density(985./SCALE_FACTOR),//avg density of human body, dividing like this is prob wrong
        Player::default(),
        ExternalImpulse::default(), //use impulses instead of velocity so controls are affected by mass
        Damping{linear_damping: 3., angular_damping: 3.},
    )).id();

    let gun = commands.spawn((GunBundle{
        object_bundle: ObjectBundle::new(Vec2::new(0., 200.), Vec2::new(0.028*SCALE_FACTOR*2., 0.185*SCALE_FACTOR), None),
        gun: Gun{},
    }, InHand)).id(); //spawn a gun with inhand component

    let joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(20.0, 55.0))
        .local_anchor2(Vec2::new(0.0, -35./2.));
    commands.entity(gun).insert(ImpulseJoint::new(player, joint));
    //manually join the gun to the player (in the future this should be done with a pickup/inv system)

}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    //buttons: Res<Input<MouseButton>>,
    mut player_info: Query<(&Player, &mut ExternalImpulse)>,
    //time_step: Res<FixedTime>,
) {
    for (player, mut ext_impulse) in &mut player_info {
        
        if keyboard_input.any_pressed([KeyCode::W, KeyCode::Up]) {
            ext_impulse.impulse.y+=100.0;
        }
        if keyboard_input.any_pressed([KeyCode::R, KeyCode::Down]) {
            ext_impulse.impulse.y-=100.0;
        }
        if keyboard_input.any_pressed([KeyCode::S, KeyCode::Right]) {
            ext_impulse.impulse.x+=100.0;
        }
        if keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]) {
            ext_impulse.impulse.x-=100.0;
        }

    }

}

#[derive(Event)]
struct PlayerAimingEvent(bool);

fn player_aiming(
    buttons: Res<Input<MouseButton>>,
    mut player_data: Query<(&mut Player, &mut ExternalImpulse, &Transform, With<RigidBody>)>,
    mut ev_playeraiming: EventWriter<PlayerAimingEvent>,
    //time_step: Res<FixedTime>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    for (mut player, mut ext_impulse, player_trans, _) in &mut player_data {
        if buttons.pressed(MouseButton::Right) {
            ev_playeraiming.send(PlayerAimingEvent(true));

            //surely this .single will never have to be changed
            let (camera, camera_transform) = camera_q.single();

            //horrific copypaste monstrosity please help i don't know how closures work
            if let Some(mouse_world_position) = windows.single().cursor_position()
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
        if buttons.just_released(MouseButton::Right) { ev_playeraiming.send(PlayerAimingEvent(false)) }
    }
}