#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy::window::PrimaryWindow;

use crate::components::Velocity;
use crate::player::components::Player;
use crate::player::systems::{PLAYER_HEIGHT, PLAYER_WIDTH};

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

pub fn confine_ball_movement(
    mut query: Query<(&mut Velocity, &mut Transform), With<Ball>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let (mut velocity, mut transform) = query.get_single_mut().unwrap();

    let half_ball_size = BALL_SIZE / 2.;
    let min = 0. + half_ball_size;
    let max = window.height() - half_ball_size;

    let mut translation = transform.translation;

    if translation.y < min {
        translation.y = min;
        velocity.y *= -1.;
    } else if translation.y > max {
        translation.y = max;
        velocity.y *= -1.;
    }

    transform.translation = translation;
}

pub fn check_for_score(
    mut query: Query<(&mut Velocity, &mut Transform), With<Ball>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let (mut velocity, mut transform) = query.get_single_mut().unwrap();

    let half_ball_size = BALL_SIZE / 2.;
    let player_one_score = window.width() + half_ball_size;
    let player_two_score = 0. - half_ball_size;

    let mut translation = transform.translation;

    if translation.x > player_one_score || translation.x < player_two_score {
        translation.x = window.width() / 2.;
        translation.y = window.height() / 2.;
        velocity.randomize();
    }

    transform.translation = translation;
}

pub fn check_player_collision(
    mut ball_query: Query<(&mut Velocity, &mut Transform), (With<Ball>, Without<Player>)>,
    player_query: Query<&Transform, With<Player>>,
) {
    let (mut ball_velocity, mut ball_transform) = ball_query.get_single_mut().unwrap();

    let half_player_width = PLAYER_WIDTH / 2.;
    let half_player_height = PLAYER_HEIGHT / 2.;
    let half_ball_size = BALL_SIZE / 2.;
    let offset_x = half_player_width + half_ball_size;
    let offset_y = half_player_height + half_ball_size;

    for player_transform in player_query.iter() {
        let collision = collide(
            ball_transform.translation,
            Vec2::splat(BALL_SIZE),
            player_transform.translation,
            Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT),
        );

        match collision {
            Some(Collision::Top) => {
                ball_transform.translation.y = player_transform.translation.y + offset_y;
                ball_velocity.y = 1.;
            }
            Some(Collision::Right) => {
                ball_transform.translation.x = player_transform.translation.x + offset_x;
                ball_velocity.x = 1.;
            }
            Some(Collision::Bottom) => {
                ball_transform.translation.y = player_transform.translation.y - offset_y;
                ball_velocity.y = -1.;
            }
            Some(Collision::Left) => {
                ball_transform.translation.x = player_transform.translation.x - offset_x;
                ball_velocity.x = -1.;
            }
            _ => (),
        }
    }
}
