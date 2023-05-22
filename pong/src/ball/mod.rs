use bevy::{prelude::*, sprite::Anchor, window::PrimaryWindow};

use self::components::Ball;

mod components;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ball);
    }
}

const BALL_SIZE: f32 = 20.0;

pub fn spawn_ball(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    let sprite = Sprite {
        color: Color::WHITE,
        flip_x: false,
        flip_y: false,
        rect: Some(Rect::new(0.0, 0.0, BALL_SIZE, BALL_SIZE)),
        anchor: Anchor::Center,
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
