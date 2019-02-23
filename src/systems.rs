use crate::vec2::Vec2;
use crate::components::*;
use pyro::{All, Read, Write};
use ggez::Context;
use ggez::graphics::{self, Color};

pub fn shoot_bullet(ctx: &mut Context, world: &mut pyro::World) {
    let screen_rect = graphics::screen_coordinates(ctx);
    let pos = Pos(Vec2::new(screen_rect.w / 2.0, screen_rect.bottom()));
    let vel = Vel(Vec2::new(0.0, -Bullet::SPEED));
    let mesh = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::Fill,
        Vec2::new(0.0, 0.0),
        1.0,
        0.0,
        Color::new(1.0, 0.0, 0.0, 1.0),
    ).expect("could not create mesh");
    world.append_components(Some((pos, vel, Bullet(0), mesh)))
}

pub fn move_balls(ctx: &mut Context, world: &mut pyro::World) {
    let dt = ggez::timer::delta(ctx).as_float_secs();

    world.matcher::<All<(Write<Pos>, Read<Vel>)>>()
        .for_each(|(pos, vel)| {
            pos.0 += vel.0 * dt as f32;
        });
}

pub fn remove_offscreen(ctx: &mut Context, world: &mut pyro::World) {
    let screen_bounds = graphics::screen_coordinates(ctx);

    let offscreen: Vec<_> = world.matcher_with_entities::<All<(Read<Pos>,)>>()
        .filter_map(|(entity, (pos,))| if pos.0.in_rect(screen_bounds) {
            None
        } else {
            Some(entity)
        }).collect();
    
    world.remove_entities(offscreen);
}

// returns the amount of ships destroyed.
pub fn handle_collision(ctx: &mut Context, world: &mut pyro::World) -> u32 {
    use std::f32;
    let mut spawn = Vec::new();
    let mut to_remove = Vec::new();
    let mut ships_destroyed = 0;

    world
        .matcher_with_entities::<All<(Read<Ship>, Read<Pos>)>>()
        .for_each(|(ship_entity, (_ship, ship_pos))| {
            world
                .matcher_with_entities::<All<(Read<Bullet>, Read<Pos>)>>()
                .for_each(|(bullet_entity, (bullet, bullet_pos))| {
                    if ship_pos.0.distance_to(bullet_pos.0) <= (bullet.size() + Ship::size(ctx))
                        && !to_remove.contains(&ship_entity)
                    {
                        to_remove.push(ship_entity);
                        to_remove.push(bullet_entity);

                        ships_destroyed += 1;

                        let to_spawn = std::cmp::min(bullet.0 + 1, 9);
                        println!("a ship and bullet hit! spawning {} bullets", to_spawn + 1);

                        for i in 0..=to_spawn {
                            let b = Bullet(to_spawn);
                            let mesh = graphics::Mesh::new_circle(
                                ctx,
                                graphics::DrawMode::Fill,
                                Vec2::new(0.0, 0.0),
                                b.size(),
                                0.1,
                                Color::new(1.0, 0.0, 0.0, 1.0),
                            ).expect("could not create mesh");
                            
                            let angle = i as f32 * (2.0 * f32::consts::PI / (to_spawn + 1) as f32);

                            spawn.push((*bullet_pos, Vel(Vec2::from_angle(angle, Bullet::SPEED)), b, mesh));
                        }
                    }
                });
        });
    
    world.remove_entities(to_remove);
    world.append_components(spawn);

    ships_destroyed
}

pub fn draw_world(ctx: &mut Context, world: &mut pyro::World) -> ggez::GameResult {
    for (pos, mesh) in world.matcher::<All<(Read<Pos>, Read<graphics::Mesh>)>>() {
        graphics::draw(
            ctx,
            mesh,
            graphics::DrawParam::default()
                .dest(pos.0),
        )?;
    }

    Ok(())
}

pub fn spawn_ships(ctx: &mut Context, world: &mut pyro::World) {
    use rand::Rng;

    let screen_bounds = graphics::screen_coordinates(ctx);
    let mut rng = rand::thread_rng();
    let count = rng.gen_range(0, 5) + 1;
    let mesh = graphics::Mesh::new_circle(
        ctx,
        graphics::DrawMode::Fill,
        Vec2::new(0.0, 0.0),
        Ship::size(ctx),
        0.1,
        Color::new(0.2, 0.3, 0.6, 1.0),
    ).expect("could not create mesh");

    println!("spawning {} ship(s)", count);

    let mut ships = Vec::with_capacity(count as usize);
    for _ in 0..count {
        let side = rng.gen();
        let x = if side {
            screen_bounds.w
        } else {
            0.0
        };
        let y = rng.gen_range(screen_bounds.h / 7.0, 6.0 * screen_bounds.h / 7.0);

        ships.push(
            (
                Pos(Vec2::new(x, y)),

                if side {
                    Vel(Vec2::new(-300.0, 0.0))
                } else {
                    Vel(Vec2::new(300.0, 0.0))
                },

                Ship,

                mesh.to_owned(),
            )
        );
    }

    world.append_components(ships);
}

pub fn bullets_on_screen(world: &mut pyro::World) -> usize {
    world.matcher::<All<(Read<Bullet>,)>>().count()
}