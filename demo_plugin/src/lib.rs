use extism_pdk::*;
use rand::Rng;

#[host_fn]
extern "ExtismHost" {
    fn turn(input: f32);
    fn move_forward(input: u32);
}

#[plugin_fn]
pub unsafe fn update() -> FnResult<()> {
    let mut rng = rand::thread_rng();
    turn(rng.gen_range(0.0..360.0))?;
    move_forward(rng.gen_range(5..100))?;
    Ok(())
}