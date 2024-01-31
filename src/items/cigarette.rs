use crate::helpers::gen_reagents;
use crate::items::Item;
use crate::reagents::container::Container;
use crate::reagents::Reagent;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Bundle)]
pub struct Cigarette {
    pub container: Container,
    pub identity: Item,
}

impl Cigarette {
    pub fn default() -> Self {
        Cigarette {
            container: Container {
                holding: gen_reagents(HashMap::from([(Reagent::Nicotine, 5), (Reagent::Toxin, 5)])),
            },
            identity: Item {
                name: Name::new("Garloid Cigarette"),
                desc: "Classic Garloid brand cigarette".to_string(),
            },
        }
    }
    pub fn robustco() -> Self {
        Cigarette {
            container: Container {
                holding: gen_reagents(HashMap::from([(Reagent::Nicotine, 10), (Reagent::Toxin, 10)])),
            },
            identity: Item {
                name: Name::new("Robustco Cigarette"),
                desc: "Now with added grass trimmings".to_string(),
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
    pub fn xuyin_omega() -> Self {
        Xuyin {
            container: Container {
                holding: vec![Reagent::Nicotine; 200],
            },
            identity: Item {
                name: Name::new("Xuyin OMEGA Mode Black Alphastar"),
                desc: "Schedule 0 controlled substance. Highly illegal worldwide.".to_string(),
            },
        }
    }
}