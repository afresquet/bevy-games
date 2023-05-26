use bevy::prelude::*;

use crate::components::Dir;

use super::systems::*;

#[derive(Component)]
pub enum Player {
    One,
    Two,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    sprite_bundle: SpriteBundle,
    direction: Dir,
    keycodes: KeyCodes,
}

#[derive(Component)]
pub struct KeyCodes {
    pub up: KeyCode,
    pub down: KeyCode,
}

impl PlayerBundle {
    pub fn new(player: Player, x: f32, y: f32) -> Self {
        let sprite = Sprite {
            color: Color::WHITE,
            rect: Some(Rect::new(0.0, 0.0, PLAYER_WIDTH, PLAYER_HEIGHT)),
            ..Default::default()
        };

        let keycodes = match player {
            Player::One => KeyCodes {
                up: KeyCode::W,
                down: KeyCode::S,
            },
            Player::Two => KeyCodes {
                up: KeyCode::Up,
                down: KeyCode::Down,
            },
        };

        PlayerBundle {
            player,
            sprite_bundle: SpriteBundle {
                sprite,
                transform: Transform::from_xyz(x, y, 0.0),
                ..Default::default()
            },
            direction: Dir::default(),
            keycodes,
        }
    }
}
