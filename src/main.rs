use bevy::{
    math::Vec3Swizzles,
    prelude::*,
    window::{PrimaryWindow, WindowResolution},
};
use bevy_console::{AddConsoleCommand, ConsoleConfiguration, ConsolePlugin};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;

pub mod body;
pub mod commands;
pub mod gun;
pub mod helpers;
pub mod object;
pub mod player;
pub mod projectile;
pub mod reagents;
pub mod ui;

use ui::*;

use crate::body::organs::{test_reagents_system, Organ};
use gun::{gun_aiming, shoot, Gun, GunBundle};
use object::ObjectBundle;

use body::hands::{handle_give_item, GiveItem, InHand};

use commands::*;

use player::{
    player_aiming, player_movement, point_player, spawn_player, PlayerAimingEvent, PlayerPointEvent,
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
            ConsolePlugin,
        ))
        .add_systems(Startup, (spawn_player, ui_load_icons))
        .add_systems(
            Update,
            (
                player_movement,
                player_aiming,
                point_player,
                gun_aiming,
                handle_give_item,
                shoot,
                ui_hand_system,
                ui_organ_system,
                //test_reagents_system,
            ),
        )
        .add_event::<PlayerAimingEvent>()
        .add_event::<PlayerPointEvent>()
        .add_event::<GiveItem>()
        .add_plugins(WorldInspectorPlugin::new())
        .add_console_command::<ExampleCommand, _>(example_command)
        //.add_plugins(EguiPlugin)
        //.insert_resource(FixedTime::new_from_secs(1.0 / 165.0))
        .run();
}

#[derive(Component, Default)]
struct Item;
#[derive(Bundle, Default)]
struct ItemBundle {
    //bundle for things that can be picked up
    obj_bundle: ObjectBundle,
}
