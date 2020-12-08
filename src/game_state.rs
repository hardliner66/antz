use crate::common::*;
use crate::components::*;
use crate::config::*;

pub struct GameState {
    pub tick: usize,
    pub config: Config,
    pub world: World,
	pub spawn: Point2,
	pub rng: ThreadRng,
}

impl GameState {
    pub fn new() -> ggez::GameResult<GameState> {
        let config: Config = toml::from_str(
            &std::fs::read_to_string("resources/config.toml")
                .unwrap_or_else(|_| include_str!("../resources/config.toml").to_string()),
        )
		.unwrap_or_default();

		let mut rng = rand::thread_rng();

		let start_x: f32 = rng.gen_range(0.0, WIDTH);
		let start_y: f32 = rng.gen_range(0.0, HEIGHT);

		let mut world = World::new();

		for _ in 0..config.general.sugar_hills {
			let sugar_x: f32 = rng.gen_range(0.0, WIDTH);
			let sugar_y: f32 = rng.gen_range(0.0, HEIGHT);

			world.spawn((Sugar, Position::new(sugar_x, sugar_y)));
		}

        let game_state = GameState {
            config,
            world,
			tick: 0,
			rng,
			spawn: Point2::new(start_x, start_y),
        };

        Ok(game_state)
    }
}
