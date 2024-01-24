use bevy_egui::{egui, EguiContexts, EguiPlugin};
use bevy::prelude::*;
use crate::body::hands::*;
use crate::body::organs::{
    liver::*,
    stomach::*,
    Organ,
    Organs,
};
use crate::player::*;

#[derive(Resource)]
pub struct UiIcons {
    frame_bg: Handle<Image>,
    frame_select: Handle<Image>,
    righthandui: Handle<Image>,
    lefthandui: Handle<Image>,
}

pub fn ui_load_icons(
    mut commands: Commands,
    server: Res<AssetServer>
) {
    let fbg: Handle<Image> = server.load("ui/frame-bg.png");
    let fsel: Handle<Image> = server.load("ui/frame-select.png");
    let rhui: Handle<Image> = server.load("ui/righthandui.png");
    let lhui: Handle<Image> = server.load("ui/lefthandui.png");

    let icons = UiIcons {
        frame_bg: fbg,
        frame_select: fsel,
        righthandui: rhui,
        lefthandui: lhui,
    };

    commands.insert_resource(icons);
}

pub fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}

pub fn ui_hand_system(
    mut contexts: EguiContexts,
    mut player_hands: Query<(
        With<Player>,
        &Hands,
    )>,
    images: Res<UiIcons>,
) {
    let bevy_texture_id = contexts.add_image(images.righthandui.clone_weak());
    egui::Window::new("Hands").show(contexts.ctx_mut(), |ui| {
        
        for (_, hands) in player_hands.get_single() { //maybe come back to this, but there should only ever be one Hands on a Player
            for hand in hands.hands.iter() {
                ui.add(egui::widgets::Image::new(
                    egui::load::SizedTexture::new(
                        bevy_texture_id, [32., 32.]
                    )
                ));
            }
            ui.label(format!("Active: {:?}", hands.get_active_held()));
        }
    });
}

pub fn ui_organ_system<T: Organ>(
    mut contexts: EguiContexts,
    organs: Query<(
        With<Player>,
        &Stomach,
        &Liver,
        &Organs<T>,
    )>,
) {
    egui::Window::new("Organs").show(contexts.ctx_mut(), |ui| {
        for (_, stomach, liver) in &organs {
            ui.label(format!("{:?}", stomach));
            ui.label(format!("{:?}", liver));
        }
    });
}