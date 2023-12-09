use antz_aux::*;

init_host_functions!();

#[plugin_fn]
pub unsafe fn on_idle() -> FnResult<()> {
    turn(rand_range(0.0, 360.0)?)?;
    move_forward(rand_range_uint(5, 100)?)?;
    Ok(())
}

#[plugin_fn]
pub unsafe fn on_near_sugar(Json(_args): Json<OnNearFoodArgs>) -> FnResult<()> {
    // turn_to(args.angle)?;
    // move_forward(args.distance)?;
    info!("on_near_sugar called");
    Ok(())
}

#[plugin_fn]
pub unsafe fn on_near_apple(Json(_args): Json<OnNearFoodArgs>) -> FnResult<()> {
    // turn_to(args.angle)?;
    // move_forward(args.distance)?;
    info!("on_near_apple called");
    Ok(())
}
