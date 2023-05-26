use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng};

#[derive(Component)]
pub struct Velocity(pub Vec3);

impl Velocity {
    pub fn random() -> Self {
        Self(Vec3::new(
            random_velocity(),
            random_velocity(),
            random_velocity(),
        ))
    }

    pub fn randomize(&mut self) {
        *self = Self::random();
    }
}

fn random_velocity() -> f32 {
    let mut rng = thread_rng();

    *[-1.0, 1.0].choose(&mut rng).unwrap()
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

#[derive(Component, Default, PartialEq, Eq)]
pub enum Dir {
    Up,
    #[default]
    Idle,
    Down,
}

impl Dir {
    pub fn should_move(&self) -> Option<f32> {
        match self {
            Self::Up => Some(1.0),
            Self::Down => Some(-1.0),
            Self::Idle => None,
        }
    }

    pub fn set(&mut self, direction: Self) {
        if *self == direction {
            return;
        }

        *self = direction;
    }
}

#[derive(Component)]
pub struct ScoreText;
