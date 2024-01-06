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
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_score)
        .add_plugins(PlayerPlugin)
        .add_plugins(BallPlugin)
        .add_systems(Update, update_score)
        .run()
}
