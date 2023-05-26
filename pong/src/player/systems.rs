use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::components::Dir;

use super::components::*;

pub const PLAYER_WIDTH: f32 = 30.0;
pub const PLAYER_HEIGHT: f32 = 200.0;
pub const HALF_PLAYER_WIDTH: f32 = PLAYER_WIDTH / 2.0;
pub const HALF_PLAYER_HEIGHT: f32 = PLAYER_HEIGHT / 2.0;
const PLAYER_SPEED: f32 = 300.0;
const GUTTER: f32 = 100.0;

pub fn spawn_players(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.single();

    let y = window.height() / 2.0;

    commands.spawn(PlayerBundle::new(Player::One, GUTTER, y));

    commands.spawn(PlayerBundle::new(Player::Two, window.width() - GUTTER, y));
}

pub fn handle_player_input(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Dir, &KeyCodes), With<Player>>,
) {
    for (mut direction, keycodes) in player_query.iter_mut() {
        let KeyCodes { up, down } = *keycodes;

        match (input.pressed(up), input.pressed(down)) {
            (true, false) => direction.set(Dir::Up),
            (false, true) => direction.set(Dir::Down),
            (_, _) => direction.set(Dir::Idle),
        };
    }
}

pub fn player_movement(
    mut player_query: Query<(&mut Transform, &Dir), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, direction) in player_query.iter_mut() {
        let Some(direction) = direction.should_move() else { continue; };

        transform.translation.y += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.single();

    let min = HALF_PLAYER_HEIGHT;
    let max = window.height() - HALF_PLAYER_HEIGHT;

    for mut transform in player_query.iter_mut() {
        transform.translation.y = transform.translation.y.clamp(min, max);
    }
}
