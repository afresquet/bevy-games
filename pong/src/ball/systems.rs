use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::Velocity;

use super::components::Ball;

const BALL_SIZE: f32 = 20.0;
const BALL_SPEED: f32 = 200.0;

pub fn spawn_ball(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    let sprite = Sprite {
        color: Color::WHITE,
        custom_size: Some(Vec2::splat(BALL_SIZE)),
        ..Default::default()
    };

    commands.spawn((
        SpriteBundle {
            sprite,
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..Default::default()
        },
        Ball,
        Velocity::random(),
    ));
}

pub fn ball_movement(mut query: Query<(&Velocity, &mut Transform), With<Ball>>, time: Res<Time>) {
    let (velocity, mut transform) = query.get_single_mut().unwrap();

    let mut transltation = transform.translation;

    transltation.x += velocity.x * BALL_SPEED * time.delta_seconds();
    transltation.y += velocity.y * BALL_SPEED * time.delta_seconds();

    transform.translation = transltation;
}
