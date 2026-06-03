use bevy::prelude::*;
use super::{Level, spawn_platform, spawn_hazard, spawn_goal, spawn_moving_platform};

pub struct Level2;

impl Level for Level2 {
    fn spawn(&self, commands: &mut Commands) {
        // Ground + ceiling — narrower than level 1
        spawn_platform(commands,   0.0, -200.0, 400.0, 20.0);
        spawn_platform(commands,   0.0,  200.0, 400.0, 20.0);

        // Left wall platforms — staircase up
        spawn_platform(commands, -220.0, -120.0, 80.0, 20.0);
        spawn_platform(commands, -160.0,  -40.0, 80.0, 20.0);
        spawn_platform(commands,  -80.0,   40.0, 80.0, 20.0);

        // Right side — floating islands
        spawn_platform(commands,  160.0,  -80.0,  80.0, 20.0);
        spawn_platform(commands,  220.0,   60.0,  60.0, 20.0);

        // Hazards on the staircase
        spawn_hazard(commands, -220.0, -100.0, 30.0, 20.0);
        spawn_hazard(commands,  -80.0,   60.0, 30.0, 20.0);

        // Hazard strip on the ground
        spawn_hazard(commands,   80.0, -180.0, 120.0, 20.0);

        // Two moving platforms — one horizontal, one vertical
        spawn_moving_platform(commands,   0.0, -80.0, 100.0, 20.0,  120.0,  -80.0, 0.5);
        spawn_moving_platform(commands, 160.0,  40.0,  60.0, 20.0,  160.0,  120.0, 0.4);

        // Goal — on top of the right floating island, need the vertical mover to reach
        spawn_goal(commands, 220.0, 90.0);
    }
}