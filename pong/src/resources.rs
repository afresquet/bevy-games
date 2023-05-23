use bevy::prelude::*;

use crate::player::components::Player;

#[derive(Resource, Default, Debug)]
pub struct Score {
    pub player_one: usize,
    pub player_two: usize,
}

impl Score {
    pub fn add_point(&mut self, player: Player) {
        match player {
            Player::One => self.player_one += 1,
            Player::Two => self.player_two += 1,
        }
    }
}
