use bevy::prelude::*;

mod components;
mod systems;

use self::systems::spawn_players;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_players);
    }
}
