use crate::{common::*, config::Config};

pub struct Ant {
    pub angle: f32,
    pub steps: i32,
    pub energy: u32,
}

impl Ant {
    pub fn new(conf: &Config) -> Self {
        Ant {
            angle: 0.0,
            steps: 0,
            energy: conf.general.base_energy,
        }
    }

    pub fn turn(&mut self, angle: f32) {
        self.angle = ((self.angle.to_degrees() + angle).rem_euclid(360.0)).to_radians();
    }

    pub fn do_move(&mut self, steps: i32) {
        self.steps = steps;
    }
}

pub struct Apple;

pub struct Sugar;

pub struct Position(pub Point2);

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Position(Point2::new(x, y))
    }
}
