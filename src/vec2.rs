#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }
    
    // from angle (radians) and magnitude
    pub fn from_angle(angle: f32, mag: f32) -> Self {
        use std::f32;

        Vec2 {
            x: f32::cos(angle) * mag,
            y: f32::sin(angle) * mag,
        }
    }
    
    pub fn in_rect(self, rect: ggez::graphics::Rect) -> bool {
           self.x >= rect.left()
        && self.x <= rect.right()
        && self.y >= rect.top()
        && self.y <= rect.bottom()
    }

    pub fn distance_to(self, other: Self) -> f32 {
        use std::f32;
        f32::hypot(self.x - other.x, self.y - other.y).abs()
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Into<ggez::mint::Point2<f32>> for Vec2 {
    fn into(self) -> ggez::mint::Point2<f32> {
        ggez::mint::Point2 { x: self.x, y: self.y }
    }
}