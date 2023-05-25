#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy::window::PrimaryWindow;

use crate::components::*;
use crate::player::components::Player;
use crate::player::systems::*;
use crate::resources::Score;

use super::components::*;

pub const BALL_SIZE: f32 = 20.0;
const HALF_BALL_SIZE: f32 = BALL_SIZE / 2.0;

pub fn spawn_ball(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();

    commands.spawn(BallBundle::new(window.width() / 2.0, window.height() / 2.0));
}

pub fn ball_movement(
    mut query: Query<(&Velocity, &mut Transform, &Speed), With<Ball>>,
    time: Res<Time>,
) {
    let (velocity, mut transform, Speed(speed)) = query.single_mut();

    let velocity = Vec2::new(velocity.x, velocity.y) * *speed * time.delta_seconds();

    transform.translation += velocity.extend(0.0);
}

pub fn confine_ball_movement(
    mut query: Query<(&mut Velocity, &mut Transform), With<Ball>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    let (mut velocity, mut transform) = query.single_mut();

    let min = HALF_BALL_SIZE;
    let max = window.height() - HALF_BALL_SIZE;

    if transform.translation.y < min {
        transform.translation.y = min;
        velocity.y = 1.0;
    } else if transform.translation.y > max {
        transform.translation.y = max;
        velocity.y = -1.0;
    }
}

pub fn check_for_score(
    mut query: Query<(&mut Velocity, &mut Transform, &mut Speed), With<Ball>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut score: ResMut<Score>,
) {
    let window = window_query.single();

    let (mut velocity, mut transform, mut speed) = query.single_mut();

    let player_one_score = window.width() + HALF_BALL_SIZE;
    let player_two_score = HALF_BALL_SIZE;

    let player = match transform.translation.x {
        x if x > player_one_score => Player::One,
        x if x < player_two_score => Player::Two,
        _ => return,
    };

    score.add_point(player);

    transform.translation = Vec3::new(window.width() / 2.0, window.height() / 2.0, 0.0);
    velocity.randomize();
    speed.reset();
}

pub fn check_player_collision(
    mut ball_query: Query<
        (&mut Velocity, &mut Transform, &mut Speed),
        (With<Ball>, Without<Player>),
    >,
    player_query: Query<&Transform, With<Player>>,
) {
    let (mut ball_velocity, mut ball_transform, mut speed) = ball_query.single_mut();

    let offset_x = HALF_PLAYER_WIDTH + HALF_BALL_SIZE;
    let offset_y = HALF_PLAYER_HEIGHT + HALF_BALL_SIZE;

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
                ball_velocity.y = 1.0;
            }
            Some(Collision::Right) => {
                ball_transform.translation.x = player_transform.translation.x + offset_x;
                ball_velocity.x = 1.0;
                speed.increase();
            }
            Some(Collision::Bottom) => {
                ball_transform.translation.y = player_transform.translation.y - offset_y;
                ball_velocity.y = -1.0;
            }
            Some(Collision::Left) => {
                ball_transform.translation.x = player_transform.translation.x - offset_x;
                ball_velocity.x = -1.0;
                speed.increase();
            }
            _ => (),
        }
    }
}
