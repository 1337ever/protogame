use crate::body::organs::*;
use crate::items::cigarette;
use crate::items::cigarette::Xuyin;
use crate::reagents::*;
use crate::body::Body;
use crate::items::Item;
use bevy::prelude::*;

#[derive(Component, Default, Debug, Reflect)]
pub struct Head {
    pub mouth: Mouth,
}

#[derive(Default, Debug, Reflect)]
pub struct Mouth {
    pub holding: Option<Entity>,
}

impl Mouth {
    pub fn with_cigarette(commands: &mut Commands) -> Self {
        Mouth {
            holding: Some(commands.spawn(cigarette::Cigarette::default()).id()),
        }
    }
    pub fn with_xuyin(commands: &mut Commands) -> Self {
        Mouth {
            holding: Some(commands.spawn(Xuyin::default()).id()),
        }
    }
    pub fn pickup(&mut self, entity: Entity) {
        if self.can_pickup() {
            self.holding = Some(entity);
        }
    }
    pub fn can_pickup(&self) -> bool {
        self.holding.is_none()
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

#[derive(Event)]
pub struct MouthGiveEvent {
    pub receiver: Entity, //which entity with mouth to give the item to
    pub target: Entity, //which entity to put in the receiver's mouth
}

pub fn handle_give_mouth(
    mut ev_mouth: EventReader<MouthGiveEvent>,
    mut head: Query<&mut Head>,
    items: Query<&Item>,
) {
    for ev in ev_mouth.read() {
        if let Ok(mut receiver) = head.get_mut(ev.receiver) {
            if items.contains(ev.target) {
                receiver.mouth.pickup(ev.target);
            }
        }
    }
}