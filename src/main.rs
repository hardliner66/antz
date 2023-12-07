#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//use std::{env, path};

mod common;
mod components;
mod config;
mod game_state;

use common::*;
use components::*;
use game_state::GameState;

use notan::draw::*;
use notan::prelude::*;

fn setup(_gfx: &mut Graphics) -> GameState {
    GameState::new()
}

fn update_ant(ant: &mut Ant) {
    let mut rng = rand::thread_rng();
    ant.turn(rng.gen_range(0.0..360.0));
    ant.do_move(rng.gen_range(5..100));
}

fn update(app: &mut App, state: &mut GameState) {
    state.tick += 1;

    if state.tick % 100 == 0 {
        state.world.spawn((
            Ant::new(&state.config),
            Position::new(state.spawn.x, state.spawn.y),
        ));
    }

    if state.tick % 1000 == 0 {
        let apple_x: f32 = state.rng.gen_range(0.0..WIDTH);
        let apple_y: f32 = state.rng.gen_range(0.0..HEIGHT);

        state.world.spawn((Apple, Position::new(apple_x, apple_y)));
    }

    for (_id, ant) in &mut state.world.query::<&mut Ant>() {
        if ant.steps == 0 {
            update_ant(ant);
        }
    }

    for (_id, (ant, pos)) in &mut state.world.query::<(&mut Ant, &mut Position)>() {
        if ant.steps > 0 {
            let x = STEP_SIZE * ant.angle.cos();
            let y = STEP_SIZE * ant.angle.sin();
            pos.0.x += x;
            pos.0.y += y;
            ant.steps -= 1;

            if pos.0.x < 0.0 || pos.0.x > WIDTH {
                pos.0.x = pos.0.x.clamp(0.0, WIDTH);
                ant.angle = 180.0_f32.to_radians() - ant.angle;
            }

            if pos.0.y < 0.0 || pos.0.y > HEIGHT {
                pos.0.y = pos.0.y.clamp(0.0, HEIGHT);
                ant.angle = 360.0_f32.to_radians() - ant.angle;
            }
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

    for (_id, (_, pos)) in &mut state.world.query::<(&Sugar, &Position)>() {
        draw.circle(10.0)
            .position(pos.0.x, pos.0.y)
            .fill_color(Color::WHITE)
            .fill();
    }

    for (_id, (_, pos)) in &mut state.world.query::<(&Apple, &Position)>() {
        draw.circle(5.0)
            .position(pos.0.x, pos.0.y)
            .fill_color(Color::from_rgb(112.0, 212.0, 0.0))
            .fill();
    }

    for (_id, (ant, pos)) in &mut state.world.query::<(&Ant, &Position)>() {
        draw.ellipse((pos.0.x, pos.0.y), (4.0, 1.5))
            .fill_color(Color::from_rgb(142.0, 178.0, 179.0))
            .fill()
            .rotate_degrees(ant.angle);
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
