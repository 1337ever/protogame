use crate::helpers::gen_reagents;
use crate::items::Item;
use crate::reagents::container::Container;
use crate::reagents::Reagent;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Bundle)]
pub struct CannedFish {
    pub container: Container,
    pub identity: Item,
}

impl CannedFish {
    pub fn default() -> Self {
        CannedFish {
            container: Container(gen_reagents(HashMap::from([
                (Reagent::Protein, 10),
                (Reagent::Omega3, 5),
                (Reagent::Salt, 5),
            ]))),
            identity: Item {
                name: Name::new("Ramen Penor Smoked Brislings"),
                desc: "A can of delicious yummy little fishies".to_string(),
            },
        }
    }
}

#[derive(Bundle)]
pub struct DonkPocket {
    pub container: Container,
    pub identity: Item,
}

impl DonkPocket {
    pub fn default() -> Self {
        DonkPocket {
            container: Container(gen_reagents(HashMap::from([
                (Reagent::Carb, 10),
                (Reagent::Sugar, 3),
                (Reagent::Salt, 5),
            ]))),
            identity: Item {
                name: Name::new("Donk Pocket"),
                desc: "Donk Co's best, a classic since 2564".to_string(),
            },
        }
    }
    //maybe this will be used to implement a mechanic so that warmed donk pockets gain bicaridine
    pub fn warm(&mut self) {
        self.container.0.push(Reagent::Bicaridine)
    }
}
