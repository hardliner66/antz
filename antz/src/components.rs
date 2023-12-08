use crate::config::Config;

use notan::math::{vec2, Vec2};

pub struct Ant {
    pub energy: u32,
}

impl Ant {
    pub fn new(conf: &Config) -> Self {
        Ant {
            energy: conf.general.base_energy,
        }
    }
}

pub struct Apple;

pub struct Sugar;

pub struct Location {
    pub pos: Vec2,
    pub angle: f32,
}

impl Location {
    pub fn new(x: f32, y: f32) -> Self {
        Self::new_with_angle(x, y, 0.0)
    }
    pub fn new_with_angle(x: f32, y: f32, angle: f32) -> Self {
        Location {
            pos: vec2(x, y),
            angle,
        }
    }
}
