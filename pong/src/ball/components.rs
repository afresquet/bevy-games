use bevy::prelude::*;

use super::systems::BALL_SIZE;
use crate::components::*;

#[derive(Component)]
pub struct Ball;

#[derive(Bundle)]
pub struct BallBundle {
    ball: Ball,
    sprite_bundle: SpriteBundle,
    velocity: Velocity,
    speed: Speed,
}

impl BallBundle {
    pub fn new(x: f32, y: f32) -> Self {
        let sprite = Sprite {
            color: Color::WHITE,
            custom_size: Some(Vec2::splat(BALL_SIZE)),
            ..Default::default()
        };

        Self {
            ball: Ball,
            sprite_bundle: SpriteBundle {
                sprite,
                transform: Transform::from_xyz(x, y, 0.0),
                ..Default::default()
            },
            velocity: Velocity::random(),
            speed: Speed::default(),
        }
    }
}
