use crate::player::Player;
use bevy::prelude::*;

//code here stolen from/based on https://github.com/mwbryant/logic-ss13-roguelike/blob/master/src/hands.rs

#[derive(Debug, Default, Clone)]
pub struct Hand {
    pub holding: Option<Entity>,
    pub health: u8,
}

impl Hand {
    pub fn default() -> Hand {
        Hand {
            holding: None,
            health: 100,
        }
    }
}

#[derive(Component, Debug, Default, Clone)]
pub struct Hands {
    pub hands: Vec<Hand>,
    pub active: Option<usize>,
}

#[derive(Component)]
pub struct InHand; //marker component for things in the player's hands
                   //probably get rid of this

#[derive(Event)]
pub struct GiveItem {
    pub receiver: Option<Entity>,
    pub item: Entity,
}

pub fn handle_give_item(
    mut commands: Commands,
    mut events: EventReader<GiveItem>,
    mut hands: Query<&mut Hands>,
    player: Query<Entity, With<Player>>,
    items: Query<&Name>,
) {
    for ev in events.read() {
        if let Ok(mut receiver) = hands.get_mut(ev.receiver.unwrap_or(player.single())) {
            if !receiver.can_pickup() {
                return;
            }

            //bug: this doesnt fire when giveitem event is sent from player setup system
            if items.contains(ev.item) {
                commands.entity(ev.item);
                receiver.pickup(ev.item);
                println!("Given item {:?} to {:?}", receiver, ev.item);
            }
        }
    }
}

impl Hands {
    pub fn swap_active(&mut self) {
        self.active = self.active.map(|index| (index + 1) % self.hands.len());
    }
    pub fn get_active(&self) -> Option<&Hand> {
        self.active.map(|index| &self.hands[index])
    }
    pub fn get_active_held(&self) -> Option<Entity> {
        self.get_active().and_then(|hand| hand.holding)
    }
    pub fn clear_active(&mut self) {
        if let Some(index) = self.active {
            self.hands[index].holding = None;
        }
    }
    //TODO: figure out wtf || syntax is (i think theyre called closures...)
    fn pickup(&mut self, entity: Entity) -> bool {
        self.active
            .and_then(|idx| self.hands.get_mut(idx))
            .filter(|hand| hand.holding.is_none())
            .map(|hand| {
                hand.holding = Some(entity);
                true
            })
            .unwrap_or(false)
    }

    pub fn can_pickup(&self) -> bool {
        self.active
            .and_then(|idx| self.hands.get(idx))
            .map(|hand| hand.holding.is_none())
            .unwrap_or(false)
    }

    pub fn human_hands() -> Self {
        Self {
            hands: vec![Hand::default(), Hand::default()],
            active: Some(0),
        }
    }
}