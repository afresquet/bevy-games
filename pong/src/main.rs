use bevy::prelude::*;

mod ball;
mod components;
mod player;
mod resources;
mod systems;

use ball::BallPlugin;
use player::PlayerPlugin;
use resources::Score;
use systems::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pong".to_string(),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .init_resource::<Score>()
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_score)
        .add_plugin(PlayerPlugin)
        .add_plugin(BallPlugin)
        .add_system(update_score)
        .run()
}
