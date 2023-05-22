use bevy::prelude::*;

mod components;
mod systems;

use self::systems::spawn_ball;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ball);
    }
}
