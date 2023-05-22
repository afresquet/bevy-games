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
