use crate::body::organs::*;
use crate::items::cigarette;
use crate::reagents::*;
use bevy::prelude::*;

#[derive(Component, Default, Debug)]
pub struct Head {
    pub mouth: Mouth,
}

#[derive(Default, Debug)]
pub struct Mouth {
    pub holding: Option<Entity>,
}

impl Mouth {
    fn with_cigarette(mut commands: Commands) -> Self {
        Mouth {
            holding: Some(commands.spawn(cigarette::Cigarette::default()).id()),
        }
    }
}

#[derive(Event)]
pub struct MouthUseEvent {
    owner: Entity, //the owner of the mouth in question
}

pub fn handle_mouth_use(
    mut ev_mouth: EventReader<MouthUseEvent>,
    mut body_data: Query<(&Organs, &Head)>,
) {
    for ev in ev_mouth.read() {
        if body_data.contains(ev.owner) {
            if let Ok((organs, head)) = body_data.get_mut(ev.owner) {}
        }
    }
}
