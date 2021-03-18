#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{env, path};

use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{
    self,
    conf::{WindowMode, WindowSetup},
    timer,
};
use ggez::{event, nalgebra::clamp};

mod common;
mod components;
mod config;
mod game_state;

use common::*;
use components::*;
use game_state::GameState;

fn update_ant(ant: &mut Ant) {
    let mut rng = rand::thread_rng();
    ant.turn(rng.gen_range(0.0, 360.0));
    ant.do_move(rng.gen_range(5, 100));
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        while timer::check_update_time(ctx, DESIRED_FPS) {
            self.tick += 1;

            if self.tick % 100 == 0 {
                self.world.spawn((
                    Ant::new(&self.config),
                    Position::new(self.spawn.x, self.spawn.y),
                ));
            }

            if self.tick % 1000 == 0 {
                let apple_x: f32 = self.rng.gen_range(0.0, WIDTH);
                let apple_y: f32 = self.rng.gen_range(0.0, HEIGHT);

                self.world.spawn((Apple, Position::new(apple_x, apple_y)));
            }

            for (_id, ant) in &mut self.world.query::<&mut Ant>() {
                if ant.steps == 0 {
                    update_ant(ant);
                }
            }

            for (_id, (ant, pos)) in &mut self.world.query::<(&mut Ant, &mut Position)>() {
                if ant.steps > 0 {
                    let x = STEP_SIZE * ant.angle.cos();
                    let y = STEP_SIZE * ant.angle.sin();
                    pos.0.x += x;
                    pos.0.y += y;
                    ant.steps -= 1;

                    if pos.0.x < 0.0 || pos.0.x > WIDTH {
                        pos.0.x = clamp(pos.0.x, 0.0, WIDTH);
                        ant.angle = 180.0_f32.to_radians() - ant.angle;
                    }

                    if pos.0.y < 0.0 || pos.0.y > HEIGHT {
                        pos.0.y = clamp(pos.0.y, 0.0, HEIGHT);
                        ant.angle = 360.0_f32.to_radians() - ant.angle;
                    }
                }
            }

            let mut to_delete = Vec::new();
            for (id, ant) in &mut self.world.query::<&mut Ant>() {
                if ant.energy > 0 {
                    ant.energy -= 1;
                } else {
                    to_delete.push(id);
                }
            }

            to_delete
                .iter()
                .for_each(|id| self.world.despawn(*id).unwrap());
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            na::Point2::new(0.0, 0.0),
            20.0,
            0.1,
            graphics::BLACK,
        )?;
        graphics::draw(ctx, &circle, (Point2::new(self.spawn.x, self.spawn.y),))?;

        for (_id, (_, pos)) in &mut self.world.query::<(&Sugar, &Position)>() {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                na::Point2::new(0.0, 0.0),
                10.0,
                1.0,
                graphics::WHITE,
            )?;
            graphics::draw(ctx, &circle, (Point2::new(pos.0.x, pos.0.y),))?;
        }

        for (_id, (_, pos)) in &mut self.world.query::<(&Apple, &Position)>() {
            let circle = graphics::Mesh::new_circle(
                ctx,
                graphics::DrawMode::fill(),
                na::Point2::new(0.0, 0.0),
                5.0,
                1.0,
                graphics::Color::from_rgb(112, 212, 0),
            )?;
            graphics::draw(ctx, &circle, (Point2::new(pos.0.x, pos.0.y),))?;
        }

        for (_id, (ant, pos)) in &mut self.world.query::<(&Ant, &Position)>() {
            // let mesh = graphics::Mesh::new_circle(
            //     ctx,
            //     graphics::DrawMode::fill(),
            //     na::Point2::new(0.0, 0.0),
            //     2.0,
            //     1.0,
            //     graphics::Color::from_rgb(142, 178, 179),
            // )?;
            let mesh = graphics::Mesh::new_ellipse(
                ctx,
                graphics::DrawMode::fill(),
                na::Point2::new(0.0, 0.0),
                4.0,
                1.5,
                1.0,
                graphics::WHITE,
            )?;
            graphics::draw(
                ctx,
                &mesh,
                (
                    Point2::new(pos.0.x, pos.0.y),
                    ant.angle,
                    graphics::Color::from_rgb(142, 178, 179),
                ),
            )?;
        }

        if self.tick % 50 == 0 {
            graphics::set_window_title(ctx, &format!("{:.0} FPS", timer::fps(ctx)));
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("antz", "ggez")
        .add_resource_path(resource_dir)
        .window_mode(WindowMode {
            width: WIDTH,
            height: HEIGHT,
            ..Default::default()
        })
        .window_setup(WindowSetup {
            vsync: false,
            ..Default::default()
        });
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut GameState::new()?;
    ggez::event::run(ctx, event_loop, state)
}
