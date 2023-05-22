use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::components::Player;

const PLAYER_WIDTH: f32 = 30.0;
const PLAYER_HEIGHT: f32 = 200.0;
const GUTTER: f32 = 100.0;

pub fn spawn_players(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    spawn_player(0, &mut commands, window);
    spawn_player(1, &mut commands, window);
}

fn spawn_player(id: usize, commands: &mut Commands, window: &Window) {
    let Some(player) = Player::from_id(id) else {
        panic!("Only two players allowed");
    };

    let x = match player {
        Player::One => GUTTER,
        Player::Two => window.width() - GUTTER,
    };

    let sprite = Sprite {
        color: Color::WHITE,
        rect: Some(Rect::new(0.0, 0.0, PLAYER_WIDTH, PLAYER_HEIGHT)),
        ..Default::default()
    };

    commands.spawn((
        SpriteBundle {
            sprite,
            transform: Transform::from_xyz(x, window.height() / 2.0, 0.0),
            ..Default::default()
        },
        player,
    ));
}
