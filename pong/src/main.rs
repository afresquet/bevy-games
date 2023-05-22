use bevy::prelude::*;

mod ball;
mod components;
mod player;
mod systems;

use ball::BallPlugin;
use player::PlayerPlugin;
use systems::spawn_camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .add_startup_system(spawn_camera)
        .add_plugin(PlayerPlugin)
        .add_plugin(BallPlugin)
        .run()
}
