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
    pub distance: f32,
    pub angle: f32,
}
