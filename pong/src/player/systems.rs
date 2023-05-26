use bevy::prelude::*;
use bevy::window::PrimaryWindow;

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

pub fn player_movement(
    input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform, &KeyCodes), With<Player>>,
    time: Res<Time>,
) {
    for (mut transform, keycodes) in player_query.iter_mut() {
        let KeyCodes { up, down } = *keycodes;

        let direction = match (input.pressed(up), input.pressed(down)) {
            (true, false) => 1.0,
            (false, true) => -1.0,
            (_, _) => continue,
        };

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
        if transform.translation.y < min {
            transform.translation.y = min;
        } else if transform.translation.y > max {
            transform.translation.y = max;
        }
    }
}
