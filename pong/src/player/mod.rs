use bevy::prelude::*;

pub mod components;
pub mod systems;

use self::systems::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_players).add_systems(
            Update,
            (
                handle_player_input,
                player_movement,
                confine_player_movement,
            )
                .chain(),
        );
    }
}
