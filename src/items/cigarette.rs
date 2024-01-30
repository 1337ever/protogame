use crate::items::Item;
use crate::reagents::container::Container;
use crate::reagents::Reagent;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct Cigarette {
    pub container: Container,
}

impl Cigarette {
    pub fn default() -> Self {
        Cigarette {
            container: Container {
                //gotta make a helper function to init this with Toxin as well
                holding: vec![Reagent::Nicotine; 5],
            },
        }
    }
    pub fn death_stick() -> Self {
        Cigarette {
            container: Container {
                holding: vec![Reagent::Toxin; 5],
            },
        }
    }
}

#[derive(Bundle)]
pub struct Xuyin {
    pub container: Container,
    pub identity: Item,
}

impl Xuyin {
    pub fn default() -> Self {
        Xuyin {
            container: Container {
                holding: vec![Reagent::Nicotine; 5],
            },
            identity: Item {
                name: Name::new("Xuyin"),
                desc: "Xuyin Brand Nicotine Pouch. Smells like satisfaction.".to_string(),
            },
        }
    }
    pub fn xuyin_xtra() -> Self {
        Xuyin {
            container: Container {
                holding: vec![Reagent::Nicotine; 10],
            },
            identity: Item {
                name: Name::new("Xuyin Xtra"),
                desc: "Xuyin Xtra Nicotine Pouch. Now featuring double the fun!".to_string(),
            },
        }
    }
}
