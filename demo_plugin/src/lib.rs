use extism_pdk::*;

#[host_fn]
extern "ExtismHost" {
    fn clear();
    fn rand() -> f32;
    fn rand_range(min: f32, max_exclusive: f32) -> f32;
    fn rand_range_int(min: i32, max_exclusive: i32) -> i32;
    fn rand_range_uint(min: u32, max_exclusive: u32) -> u32;
    fn turn(input: f32);
    fn move_forward(input: u32);
}

#[plugin_fn]
pub unsafe fn on_idle() -> FnResult<()> {
    turn(rand_range(0.0, 360.0)?)?;
    move_forward(rand_range_uint(5, 100)?)?;
    Ok(())
}
