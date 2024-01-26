use crate::agent::player::*;
use crate::body::hands::*;
use crate::body::organs::{liver::*, stomach::*, Organ, Organs};
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

#[derive(Resource)]
pub struct UiIcons {
    frame_bg: Handle<Image>,
    frame_select: Handle<Image>,
    righthandui: Handle<Image>,
    lefthandui: Handle<Image>,
}

pub fn ui_load_icons(mut commands: Commands, server: Res<AssetServer>) {
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
    player_hands: Query<(With<Player>, &Hands)>,
    images: Res<UiIcons>,
    names: Query<&Name>,
) {
    let bevy_texture_id = contexts.add_image(images.righthandui.clone_weak());
    egui::Window::new("Hands").show(contexts.ctx_mut(), |ui| {
        for (_, hands) in player_hands.get_single() {
            //maybe come back to this, but there should only ever be one Hands on a Player
            for hand in hands.hands.iter() {
                ui.add(egui::widgets::Image::new(egui::load::SizedTexture::new(
                    bevy_texture_id,
                    [32., 32.],
                )));
            }
            ui.label(format!(
                "Active: {}",
                names.get(hands.get_active_held().unwrap()).unwrap()
            ));
        }
    });
}

pub fn ui_organ_system(
    mut contexts: EguiContexts,
    organ_query: Query<(With<Player>, &Organs)>,
    stomach_query: Query<(&Stomach)>,
    names: Query<&Name>,
) {
    egui::Window::new("Organs").show(contexts.ctx_mut(), |ui| {
        for (_, organs) in &organ_query {
            for organ in &organs.organs {
                ui.label(format!("{}", names.get(*organ).unwrap()));
            }
        }
        for stomach in &stomach_query {
            ui.label(format!("{:?}", stomach.list_reagents()));
        }
    });
}
