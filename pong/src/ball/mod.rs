use bevy::prelude::*;

mod components;
mod systems;

use self::systems::*;

pub struct BallPlugin;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum PhysicsSet {
    Movement,
    Collision,
}

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.configure_set(PhysicsSet::Movement.before(PhysicsSet::Collision))
            .add_startup_system(spawn_ball)
            .add_system(ball_movement.in_set(PhysicsSet::Movement))
            .add_systems(
                (
                    check_player_collision,
                    confine_ball_movement,
                    check_for_score,
                )
                    .in_set(PhysicsSet::Collision),
            );
    }
}
