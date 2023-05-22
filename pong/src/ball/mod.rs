use bevy::prelude::*;

mod components;
mod systems;

use self::systems::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_ball).add_system(ball_movement);
    }
}
