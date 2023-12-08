#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//use std::{env, path};

mod common;
mod components;
mod config;
mod game_state;

use common::*;
use components::*;
use game_state::GameState;

use extism::*;
use notan::draw::*;
use notan::prelude::*;

host_fn!(clear(user_data: Vec<Command>;) {
    let data = user_data.get()?;
    let mut data = data.lock().unwrap();
    data.clear();
    Ok(())
});

host_fn!(rand() -> f32 {
    Ok(random())
});

host_fn!(rand_range(min: f32, max_exclusive: f32) -> f32 {
    let mut rng = rand::thread_rng();
    Ok(rng.gen_range(min..max_exclusive))
});

host_fn!(rand_range_int(min: i32, max_exclusive: i32) -> f32 {
    let mut rng = rand::thread_rng();
    Ok(rng.gen_range(min..max_exclusive))
});

host_fn!(turn(user_data: Vec<Command>; value: f32) {
    let data = user_data.get()?;
    let mut data = data.lock().unwrap();
    data.push(Command::Turn(value));
    Ok(())
});

host_fn!(move_forward(user_data: Vec<Command>; value: u32) {
    let data = user_data.get()?;
    let mut data = data.lock().unwrap();
    data.push(Command::Move(value));
    Ok(())
});

fn setup(_gfx: &mut Graphics) -> GameState {
    let url = Wasm::file("./target/wasm32-wasi/release/demo_plugin.wasm");
    let manifest = Manifest::new([url]);
    let command_store = UserData::new(Vec::new());
    let plugin = extism::PluginBuilder::new(&manifest)
        .with_wasi(true)
        .with_function("clear", [], [], command_store.clone(), clear)
        .with_function("turn", [PTR], [], command_store.clone(), turn)
        .with_function("rand", [], [PTR], UserData::default(), rand)
        .with_function(
            "rand_range",
            [PTR, PTR],
            [PTR],
            UserData::default(),
            rand_range,
        )
        .with_function(
            "rand_range_int",
            [PTR, PTR],
            [PTR],
            UserData::default(),
            rand_range_int,
        )
        .with_function(
            "move_forward",
            [PTR],
            [],
            command_store.clone(),
            move_forward,
        )
        .build()
        .unwrap();
    GameState::new(plugin, command_store.clone())
}

fn ant_on_idle(state: &GameState, commands: &mut Vec<Command>) {
    let mut plugin = state.plugin.borrow_mut();
    (*plugin).call::<(), ()>("on_idle", ()).unwrap();
    commands.extend(state.user_data.get().unwrap().lock().unwrap().clone());
}

fn update(app: &mut App, state: &mut GameState) {
    state.tick += 1;

    if state.tick % 100 == 0 {
        state.world.spawn((
            Ant::new(&state.config),
            Location::new(state.spawn.x, state.spawn.y),
        ));
    }

    if state.tick % 1000 == 0 {
        let apple_x: f32 = state.rng.gen_range(0.0..WIDTH);
        let apple_y: f32 = state.rng.gen_range(0.0..HEIGHT);

        state.world.spawn((Apple, Location::new(apple_x, apple_y)));
    }

    for (_id, (_ant, commands)) in &mut state.world.query::<(&mut Ant, &mut Vec<Command>)>() {
        if commands.is_empty() {
            ant_on_idle(state, commands);
        }
    }

    for (_id, (_, loc, commands)) in &mut state
        .world
        .query::<(&mut Ant, &mut Location, &mut Vec<Command>)>()
    {
        match commands.last_mut() {
            Some(Command::Turn(angle)) => {
                loc.angle = ((loc.angle.to_degrees() + *angle).rem_euclid(360.0)).to_radians();
            }
            Some(Command::Move(steps)) => {
                if *steps > 0 {
                    let x = STEP_SIZE * loc.angle.cos();
                    let y = STEP_SIZE * loc.angle.sin();
                    loc.pos.x += x;
                    loc.pos.y += y;
                    *steps -= 1;

                    if loc.pos.x < 0.0 || loc.pos.x > WIDTH {
                        loc.pos.x = loc.pos.x.clamp(0.0, WIDTH);
                        loc.angle = 180.0_f32.to_radians() - loc.angle;
                    }

                    if loc.pos.y < 0.0 || loc.pos.y > HEIGHT {
                        loc.pos.y = loc.pos.y.clamp(0.0, HEIGHT);
                        loc.angle = 360.0_f32.to_radians() - loc.angle;
                    }
                } else {
                    _ = commands.pop();
                }
            }
            None => todo!(),
        }
    }

    let mut to_delete = Vec::new();
    for (id, ant) in &mut state.world.query::<&mut Ant>() {
        if ant.energy > 0 {
            ant.energy -= 1;
        } else {
            to_delete.push(id);
        }
    }

    to_delete
        .iter()
        .for_each(|id| state.world.despawn(*id).unwrap());

    if state.tick % 50 == 0 {
        let fps = app.timer.fps();
        app.window().set_title(&format!("{:.0} FPS", fps))
    }
}

fn draw(gfx: &mut Graphics, state: &mut GameState) {
    let mut draw = gfx.create_draw();

    draw.clear([0.1, 0.2, 0.3, 1.0].into());

    draw.circle(20.0)
        .position(state.spawn.x, state.spawn.y)
        .fill_color(Color::BLACK)
        .fill();

    for (_id, (_, loc)) in &mut state.world.query::<(&Sugar, &Location)>() {
        draw.circle(10.0)
            .position(loc.pos.x, loc.pos.y)
            .fill_color(Color::WHITE)
            .fill();
    }

    for (_id, (_, loc)) in &mut state.world.query::<(&Apple, &Location)>() {
        draw.circle(5.0)
            .position(loc.pos.x, loc.pos.y)
            .fill_color(Color::from_rgb(112.0, 212.0, 0.0))
            .fill();
    }

    for (_id, (_, loc)) in &mut state.world.query::<(&Ant, &Location)>() {
        draw.ellipse((loc.pos.x, loc.pos.y), (4.0, 1.5))
            .fill_color(Color::from_rgb(142.0, 178.0, 179.0))
            .fill()
            .rotate_degrees(loc.angle);
    }

    gfx.render(&draw);
}

#[notan_main]
fn main() -> Result<(), String> {
    // let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
    //     let mut path = path::PathBuf::from(manifest_dir);
    //     path.push("resources");
    //     path
    // } else {
    //     path::PathBuf::from("./resources")
    // };

    notan::init_with(setup)
        .update(update)
        .draw(draw)
        .add_config(DrawConfig)
        .build()
}
