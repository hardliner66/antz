use crate::config::Config;

use derive_more::{Deref, DerefMut};
use notan::math::{vec2, Vec2};

pub struct Ant;

#[derive(Deref, DerefMut)]
pub struct Energy(pub u32);
impl Energy {
    pub fn new(conf: &Config) -> Self {
        Self(conf.general.base_energy)
    }
}

pub struct Apple;

pub struct Sugar;

#[derive(Deref, DerefMut)]
pub struct Amount(pub u32);

#[derive(Deref, DerefMut)]
pub struct Position(pub Vec2);

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self(vec2(x, y))
    }
}

#[derive(Deref, DerefMut)]
pub struct Orientation(pub f32);
