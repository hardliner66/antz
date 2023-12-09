pub use extism_pdk;
pub use extism_pdk::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AntState {
    pub energy: u32,
}

impl AntState {
    pub fn new(energy: u32) -> Self {
        Self { energy }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OnNearFoodArgs {
    pub this: AntState,
    pub distance: u32,
    pub angle: f32,
}

#[macro_export]
macro_rules! init_host_functions {
    () => {
        #[host_fn]
        extern "ExtismHost" {
            pub fn clear();
            pub fn rand() -> f32;
            pub fn rand_range(min: f32, max_exclusive: f32) -> f32;
            pub fn rand_range_int(min: i32, max_exclusive: i32) -> i32;
            pub fn rand_range_uint(min: u32, max_exclusive: u32) -> u32;
            pub fn turn(input: f32);
            pub fn turn_to(input: f32);
            pub fn move_forward(input: u32);
        }
    };
}
