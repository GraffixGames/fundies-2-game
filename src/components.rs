use crate::vec2::Vec2;
use ggez::Context;

#[derive(Copy, Clone)]
pub struct Pos(pub Vec2);
pub struct Vel(pub Vec2);

pub struct Bullet(pub u32);

impl Bullet {
    pub const SPEED: f32 = 400.0;
}

impl Bullet {
    pub fn size(&self) -> f32 {
        10.0_f32.min(self.0 as f32)
    }
}

pub struct Ship;

impl Ship {
    pub fn size(ctx: &Context) -> f32 {
        ggez::graphics::screen_coordinates(ctx).h / 35.0
    }
}