use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct General {
	pub sugar_hills: u8,
	pub base_energy: u32,
}

#[derive(Deserialize, Debug, Default)]
pub struct Config {
	pub general: General,
}
