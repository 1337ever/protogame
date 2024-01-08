use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::object::ObjectBundle;


#[derive(Bundle, Default)]
pub struct BulletBundle {
    obj_bundle: ObjectBundle,
    bullet: Bullet,
}
//9mm bullet length: 10.54mm
//width: 9.65mm
//weight: 5.2g
#[derive(Component, Default)]
pub struct Bullet;