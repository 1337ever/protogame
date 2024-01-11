use bevy::{
    prelude::*,
    window::{WindowResolution, PrimaryWindow},
    math::Vec3Swizzles,
};
use bevy_rapier2d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod hands;
pub mod gun;
pub mod projectile;
pub mod object;
pub mod player;

use object::ObjectBundle;
use gun::{
    Gun,
    gun_aiming,
    GunBundle,
    shoot,
};

use hands::{
    InHand,
    handle_give_item,
    GiveItem,
};

use player::{
    spawn_player,
    player_movement,
    player_aiming,
    PlayerAimingEvent,
};

pub const SCALE_FACTOR: f32 = 50.;

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
        .add_systems(
            Update, 
            (
                        player_movement, 
                        player_aiming, 
                        gun_aiming, 
                        handle_give_item,
                        shoot
                    )
        )
        .add_event::<PlayerAimingEvent>()
        .add_event::<GiveItem>()
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