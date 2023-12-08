use std::cell::RefCell;

use crate::common::*;
use crate::components::*;
use crate::config::*;

use extism::*;
use notan::math::Vec2;
use notan::AppState;

#[derive(AppState)]
pub struct GameState {
    pub tick: usize,
    pub config: Config,
    pub world: World,
    pub spawn: Vec2,
    pub rng: ThreadRng,
    pub plugin: RefCell<Plugin>,
    pub user_data: UserData<Vec<Command>>,
}

impl GameState {
    pub fn new(plugin: Plugin, user_data: UserData<Vec<Command>>) -> GameState {
        let config: Config = toml::from_str(
            &std::fs::read_to_string("resources/config.toml")
                .unwrap_or_else(|_| include_str!("../resources/config.toml").to_string()),
        )
        .unwrap_or_default();

        let mut rng = rand::thread_rng();

        let start_x: f32 = rng.gen_range(0.0..WIDTH);
        let start_y: f32 = rng.gen_range(0.0..HEIGHT);

        let mut world = World::new();

        for _ in 0..config.general.sugar_hills {
            let sugar_x: f32 = rng.gen_range(0.0..WIDTH);
            let sugar_y: f32 = rng.gen_range(0.0..HEIGHT);

            world.spawn((Sugar, Location::new(sugar_x, sugar_y)));
        }

        let game_state = GameState {
            config,
            world,
            tick: 0,
            rng,
            spawn: Vec2::new(start_x, start_y),
            plugin: RefCell::new(plugin),
            user_data,
        };

        game_state
    }
}
