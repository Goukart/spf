use bevy::prelude::Vec3;

pub struct Bullet {
    pub pos: Vec3,
    pub vel: Vec3,
    pub shot_at: f64,
    pub collided: bool,
}
