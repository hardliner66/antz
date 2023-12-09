#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;
//use std::{env, path};

mod common;
mod components;
mod config;
mod game_state;

use antz_aux::AntState;
use antz_aux::OnNearFoodArgs;

use clap::Parser;
use common::*;
use components::*;
use extism::convert::Json;
use game_state::GameState;

use extism::*;
use log::info;
use notan::draw::*;
use notan::prelude::*;
use tracing_subscriber::{filter::LevelFilter, EnvFilter};

host_fn!(clear(user_data: Vec<Command>;) {
    let data = user_data.get()?;
    let mut data = data.lock().unwrap();
    data.clear();
    Ok(())
});

host_fn!(rand(thread_rng: ThreadRng;) -> f32 {
    let rng = thread_rng.get()?;
    let mut rng = rng.lock().unwrap();
    Ok(rng.gen())
});

host_fn!(rand_range(thread_rng: ThreadRng; min: f32, max_exclusive: f32) -> f32 {
    let rng = thread_rng.get()?;
    let mut rng = rng.lock().unwrap();
    Ok(rng.gen_range(min..max_exclusive))
});

host_fn!(rand_range_int(thread_rng: ThreadRng; min: i32, max_exclusive: i32) -> i32 {
    let rng = thread_rng.get()?;
    let mut rng = rng.lock().unwrap();
    Ok(rng.gen_range(min..max_exclusive))
});

host_fn!(rand_range_uint(thread_rng: ThreadRng; min: u32, max_exclusive: u32) -> u32 {
    let rng = thread_rng.get()?;
    let mut rng = rng.lock().unwrap();
    Ok(rng.gen_range(min..max_exclusive))
});

host_fn!(turn(user_data: Vec<Command>; value: f32) {
    let data = user_data.get()?;
    let mut data = data.lock().unwrap();
    data.push(Command::Turn(value));
    Ok(())
});

host_fn!(turn_to(user_data: Vec<Command>; value: f32) {
    let data = user_data.get()?;
    let mut data = data.lock().unwrap();
    data.push(Command::TurnTo(value));
    Ok(())
});

host_fn!(move_forward(user_data: Vec<Command>; value: u32) {
    let data = user_data.get()?;
    let mut data = data.lock().unwrap();
    data.push(Command::Move(value));
    Ok(())
});

fn setup(_gfx: &mut Graphics) -> GameState {
    let Args { wasm_file } = Args::parse();
    let url = Wasm::file(wasm_file);
    let manifest = Manifest::new([url]);
    let command_store = UserData::new(Vec::new());
    let rng = UserData::new(thread_rng());
    let plugin = extism::PluginBuilder::new(&manifest)
        .with_wasi(false)
        .with_function("clear", [], [], command_store.clone(), clear)
        .with_function("turn", [PTR], [], command_store.clone(), turn)
        .with_function("turn_to", [PTR], [], command_store.clone(), turn_to)
        .with_function("rand", [], [PTR], rng.clone(), rand)
        .with_function("rand_range", [PTR, PTR], [PTR], rng.clone(), rand_range)
        .with_function(
            "rand_range_int",
            [PTR, PTR],
            [PTR],
            rng.clone(),
            rand_range_int,
        )
        .with_function(
            "rand_range_uint",
            [PTR, PTR],
            [PTR],
            rng.clone(),
            rand_range_uint,
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
    if !plugin.function_exists("on_idle") {
        info!("Ant does not implement on_idle");
        return;
    }
    (*plugin).call::<(), ()>("on_idle", ()).unwrap();
    commands.extend(state.user_data.get().unwrap().lock().unwrap().clone());
}

fn ant_on_near_food(
    state: &GameState,
    ant_pos: &Position,
    food_pos: &Position,
    ant_state: AntState,
    is_apple: bool,
    commands: &mut Vec<Command>,
) {
    let function_name = if is_apple {
        "on_near_apple"
    } else {
        "on_near_sugar"
    };
    let mut plugin = state.plugin.borrow_mut();
    if !plugin.function_exists(function_name) {
        info!("Ant does not implement {function_name}");
        return;
    }
    let distance = (**ant_pos).distance(**food_pos);
    let angle = (**ant_pos).angle_between(**food_pos).to_degrees();
    (*plugin)
        .call::<Json<OnNearFoodArgs>, ()>(
            function_name,
            Json(OnNearFoodArgs {
                distance: distance as u32,
                angle,
                this: ant_state,
            }),
        )
        .unwrap();
    let user_data = state.user_data.get().unwrap();
    let user_data = user_data.lock().unwrap();
    if !user_data.is_empty() {
        commands.clear();
        commands.extend(user_data.clone());
    }
}

enum FoodType {
    Apple,
    Sugar,
}

enum ComponentChange {
    AddSeen(Entity, FoodType),
    RemoveSeen(Entity, FoodType),
}

fn update(app: &mut App, state: &mut GameState) {
    state.tick += 1;

    let mut to_delete = Vec::new();
    for (id, (_, energy)) in &mut state.world.query::<(&Ant, &mut Energy)>() {
        if **energy > 0 {
            **energy -= 1;
        } else {
            to_delete.push(id);
        }
    }

    to_delete
        .iter()
        .for_each(|id| state.world.despawn(*id).unwrap());

    if state.tick % 100 == 0 {
        state.world.spawn((
            Ant,
            Energy::new(&state.config),
            Position::new(state.spawn.x, state.spawn.y),
            Orientation(0.0),
            CommandList::new(),
        ));
    }

    if state.tick % 1000 == 0 {
        let apple_x: f32 = state.rng.gen_range(0.0..WIDTH);
        let apple_y: f32 = state.rng.gen_range(0.0..HEIGHT);

        state.world.spawn((Apple, Position::new(apple_x, apple_y)));
    }

    for (_id, commands) in &mut state.world.query::<With<&mut Vec<Command>, &Ant>>() {
        if commands.is_empty() {
            ant_on_idle(state, commands);
        }
    }

    let mut component_changes = Vec::new();
    for (id, (ant_pos, energy, commands)) in &mut state
        .world
        .query::<With<(&Position, &Energy, &mut CommandList), &Ant>>()
    {
        for (_id, (pos, or)) in &mut state.world.query::<(&Position, Or<&Sugar, &Apple>)>() {
            let is_apple = or.right().is_some();
            let has_food_seen = if is_apple {
                state.world.entity(id).unwrap().has::<Seen<Apple>>()
            } else {
                state.world.entity(id).unwrap().has::<Seen<Sugar>>()
            };
            let distance = (**ant_pos).distance(**pos);
            if has_food_seen && distance > 10.0 {
                component_changes.push(ComponentChange::RemoveSeen(
                    id,
                    if is_apple {
                        FoodType::Apple
                    } else {
                        FoodType::Sugar
                    },
                ));
            } else if !has_food_seen && distance <= 10.0 {
                ant_on_near_food(
                    state,
                    ant_pos,
                    pos,
                    AntState::new(**energy),
                    is_apple,
                    commands,
                );
                component_changes.push(ComponentChange::AddSeen(
                    id,
                    if is_apple {
                        FoodType::Apple
                    } else {
                        FoodType::Sugar
                    },
                ));
            }
        }
    }

    for change in component_changes {
        match change {
            ComponentChange::AddSeen(id, FoodType::Apple) => {
                state.world.insert(id, (Seen::<Apple>::default(),)).unwrap()
            }
            ComponentChange::AddSeen(id, FoodType::Sugar) => {
                state.world.insert(id, (Seen::<Sugar>::default(),)).unwrap()
            }
            ComponentChange::RemoveSeen(id, FoodType::Apple) => {
                _ = state.world.remove_one::<Seen<Apple>>(id)
            }
            ComponentChange::RemoveSeen(id, FoodType::Sugar) => {
                _ = state.world.remove_one::<Seen<Sugar>>(id)
            }
        };
    }

    for (_id, (_, pos, ori, commands)) in
        &mut state
            .world
            .query::<(&Ant, &mut Position, &mut Orientation, &mut Vec<Command>)>()
    {
        let done = match commands.last_mut() {
            Some(Command::Turn(angle)) => {
                **ori = ((ori.to_degrees() + *angle).rem_euclid(360.0)).to_radians();
                true
            }
            Some(Command::TurnTo(angle)) => {
                **ori = (*angle).rem_euclid(360.0).to_radians();
                true
            }
            Some(Command::Move(steps)) => {
                if *steps > 0 {
                    let x = STEP_SIZE * ori.cos();
                    let y = STEP_SIZE * ori.sin();
                    pos.x += x;
                    pos.y += y;
                    *steps -= 1;

                    if pos.x < 0.0 || pos.x > WIDTH {
                        pos.x = pos.x.clamp(0.0, WIDTH);
                        **ori = 180.0_f32.to_radians() - **ori;
                    }

                    if pos.y < 0.0 || pos.y > HEIGHT {
                        pos.y = pos.y.clamp(0.0, HEIGHT);
                        **ori = 360.0_f32.to_radians() - **ori;
                    }
                    false
                } else {
                    true
                }
            }
            None => false,
        };
        if done {
            _ = commands.pop();
        }
    }

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

    for (_id, (_, pos)) in &mut state.world.query::<(&Sugar, &Position)>() {
        draw.circle(10.0)
            .position(pos.x, pos.y)
            .fill_color(Color::WHITE)
            .fill();
        draw.circle(1.0)
            .position(pos.x, pos.y)
            .fill_color(Color::BLACK)
            .fill();
    }

    for (_id, (_, pos)) in &mut state.world.query::<(&Apple, &Position)>() {
        draw.circle(5.0)
            .position(pos.x, pos.y)
            .fill_color(Color::from_rgb(112.0, 212.0, 0.0))
            .fill();
        draw.circle(1.0)
            .position(pos.x, pos.y)
            .fill_color(Color::BLACK)
            .fill();
    }

    for (_id, (_, pos, ori)) in &mut state.world.query::<(&Ant, &Position, &Orientation)>() {
        draw.ellipse((pos.x, pos.y), (4.0, 1.5))
            .fill_color(Color::from_rgb(142.0, 178.0, 179.0))
            .fill()
            .rotate_degrees((**ori).to_degrees());
        draw.circle(10.0)
            .position(pos.x, pos.y)
            .stroke_color(Color::BLACK)
            .stroke(1.0);
    }

    gfx.render(&draw);
}

fn setup_logger() -> anyhow::Result<()> {
    let filter = EnvFilter::try_from_env("RUST_LOG")
        .unwrap_or_default()
        .add_directive(LevelFilter::INFO.into());
    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbreviated log format
        .pretty()
        .with_timer(tracing_subscriber::fmt::time::uptime())
        .with_env_filter(filter)
        // Display source code file paths
        .with_file(false)
        // Display source code line numbers
        .with_line_number(false)
        // Display the thread ID an event was recorded on
        .with_thread_ids(false)
        // Don't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    Ok(())
}

#[derive(Parser)]
struct Args {
    wasm_file: PathBuf,
}

#[notan_main]
fn main() -> Result<(), String> {
    setup_logger().unwrap();
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
        .add_config(WindowConfig::default().set_vsync(true))
        .build()
}
