#![feature(duration_float)]

use ggez::graphics::{self, Color};
use ggez::Context;
use ggez::input::keyboard::{KeyCode, KeyMods};
use std::time::{Duration, Instant};

mod vec2;
use vec2::Vec2;
mod components;
mod systems;
use systems::*;

struct GameState {
    world: pyro::World,
    bullets_left: u32,
    ships_destroyed: u32,
    last_ship_spawn: Instant,
}

impl GameState {
    const SHIP_SPAWN_DURATION: Duration = Duration::from_millis(750);
}

impl GameState {
    pub fn new(bullets_left: u32) -> Self {
        GameState {
            world: pyro::World::new(),
            bullets_left,
            ships_destroyed: 0,
            last_ship_spawn: Instant::now(),
        }
    }
}

impl ggez::event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if self.last_ship_spawn.elapsed() > Self::SHIP_SPAWN_DURATION {
            spawn_ships(ctx, &mut self.world);
            self.last_ship_spawn = Instant::now();
        }

        move_balls(ctx, &mut self.world);
        remove_offscreen(ctx, &mut self.world);
        self.ships_destroyed += handle_collision(ctx, &mut self.world);

        if self.bullets_left == 0 && bullets_on_screen(&mut self.world) == 0 {
            self.quit_event(ctx);
            ggez::quit(ctx);
        }
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let screen_bounds = graphics::screen_coordinates(ctx);
        ggez::graphics::clear(ctx, ggez::graphics::Color::new(0.0,0.0,0.0,1.0));

        draw_world(ctx, &mut self.world)?;

        graphics::queue_text(
            ctx,
            &graphics::Text::new(format!("bullets_left: {}, ships_destroyed: {}", self.bullets_left, self.ships_destroyed)),
            Vec2::new(0.0, 0.0),
            Some(Color::new(1.0, 1.0, 1.0, 1.0))
        );
        
        graphics::draw_queued_text(
            ctx, 
            graphics::DrawParam::default()
                .dest(Vec2::new(10.0, screen_bounds.h - 20.0))
        )?;

        ggez::graphics::present(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, _: KeyMods, repeat: bool) {
        if keycode == KeyCode::Escape {
            self.quit_event(ctx);
            ggez::quit(ctx);
        }

        if !repeat && KeyCode::Space == keycode && self.bullets_left > 0 {
            shoot_bullet(ctx, &mut self.world);
            self.bullets_left -= 1;
        }
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> bool {
        println!("You destroyed {} ships!", self.ships_destroyed);
        false
    }
}

// test comment
fn main() {
    let (mut ctx, mut events_loop) = ggez::ContextBuilder::new("fundies 2 game", "Noah Graff")
        .window_setup(
            ggez::conf::WindowSetup::default()
                .title("Fundies 2 Game")
        )
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(1000.0, 600.0)
        )
        .build()
        .expect("could not initialize ggez");

    ggez::event::run(&mut ctx, &mut events_loop, &mut GameState::new(10))
        .expect("exited with error");
}
