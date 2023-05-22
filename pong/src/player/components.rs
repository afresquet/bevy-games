use bevy::prelude::*;

#[derive(Component)]
pub enum Player {
    One,
    Two,
}

impl Player {
    pub fn from_id(id: usize) -> Option<Self> {
        match id {
            0 => Some(Player::One),
            1 => Some(Player::Two),
            _ => None,
        }
    }
}
