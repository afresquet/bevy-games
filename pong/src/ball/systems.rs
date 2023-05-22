use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::Ball;

const BALL_SIZE: f32 = 20.0;

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
    ));
}
