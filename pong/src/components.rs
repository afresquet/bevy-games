use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Velocity {
    pub fn random() -> Self {
        Self {
            x: random_velocity(),
            y: random_velocity(),
            z: random_velocity(),
        }
    }

    pub fn randomize(&mut self) {
        self.x = random_velocity();
        self.y = random_velocity();
        self.z = random_velocity();
    }
}

fn random_velocity() -> f32 {
    if rand::random() {
        1.
    } else {
        -1.
    }
}

#[derive(Component)]
pub struct Speed(pub f32);

impl Speed {
    pub fn increase(&mut self) {
        self.0 *= 1.10;
    }

    pub fn reset(&mut self) {
        self.0 = 200.0;
    }
}

impl Default for Speed {
    fn default() -> Self {
        Self(200.0)
    }
}

#[derive(Component)]
pub struct ScoreText;
