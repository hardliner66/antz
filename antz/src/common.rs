pub use hecs::*;

use std::ops::{Add, Div, Mul, Sub};

pub use crate::game_state::*;
pub use rand::prelude::*;

pub const WIDTH: f32 = 1280.0;
// pub const MIDDLE_X: f32 = WIDTH / 2.0;
pub const HEIGHT: f32 = 720.0;
// pub const MIDDLE_Y: f32 = HEIGHT / 2.0;
// pub const DESIRED_FPS: u32 = 60;
pub const STEP_SIZE: f32 = 2.0;

#[derive(Debug, Clone)]
pub enum Command {
    Move(u32),
    Turn(f32),
    TurnTo(f32),
}

pub type CommandList = Vec<Command>;

#[inline(always)]
#[allow(unused)]
pub fn map_range<
    T: Mul<Output = T> + Add<Output = T> + Sub<Output = T> + Div<Output = T> + Copy,
>(
    val: T,
    start1: T,
    end1: T,
    start2: T,
    end2: T,
) -> T {
    (val - start1) / (end1 - start1) * (end2 - start2) + start2
}
