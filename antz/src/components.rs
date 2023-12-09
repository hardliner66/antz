use std::marker::PhantomData;

use crate::config::Config;

use derive_more::{Deref, DerefMut};
use notan::math::{vec2, Vec2};

#[derive(Debug, Default, Clone)]
pub struct Ant;

#[derive(Debug, Deref, DerefMut)]
pub struct Energy(pub u32);
impl Energy {
    pub fn new(conf: &Config) -> Self {
        Self(conf.general.base_energy)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Apple;
#[derive(Debug, Default, Clone)]
pub struct Sugar;

#[derive(Debug, Default, Clone)]
pub struct Seen<T>(PhantomData<T>);
#[derive(Debug, Deref, DerefMut)]
pub struct Amount(pub u32);

#[derive(Debug, Deref, DerefMut)]
pub struct Position(pub Vec2);

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self(vec2(x, y))
    }
}

#[derive(Debug, Deref, DerefMut)]
pub struct Orientation(pub f32);
