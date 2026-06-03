use bevy::prelude::*;
use super::{Level, spawn_platform, spawn_hazard, spawn_goal, spawn_moving_platform};

pub struct Level3;

impl Level for Level3 {
    fn spawn(&self, commands: &mut Commands) {
        // Floor (gravity normal) — spawn is at (0, 150) so player falls here
        spawn_platform(commands, -100.0, -200.0, 200.0, 20.0);

        // Safe starting platform directly under spawn point
        spawn_platform(commands,   0.0,   40.0, 100.0, 10.0);  

        // Ceiling (gravity flipped)
        spawn_platform(commands,  100.0,  200.0, 200.0, 20.0);

        // Middle corridor hazards — moved away from spawn path
        spawn_hazard(commands, -100.0,  -30.0, 30.0, 20.0);  
        spawn_hazard(commands,  120.0,   50.0, 20.0, 10.0);  
        spawn_hazard(commands,  100.0,  -60.0, 30.0, 20.0);
        spawn_hazard(commands, -100.0,   60.0, 30.0, 6.0);

        // Small platforms to land on after flipping
        spawn_platform(commands,  -20.0, -60.0,  80.0, 10.0);
        spawn_platform(commands,   80.0,  60.0,  80.0, 10.0);
        spawn_platform(commands,  -80.0, 120.0,  80.0, 10.0);
        spawn_platform(commands,  160.0, -120.0, 80.0, 10.0);

        // Moving platforms
        spawn_moving_platform(commands,  40.0, -140.0, 80.0, 20.0,  160.0, -140.0, 0.6);
        spawn_moving_platform(commands, -40.0,  140.0, 80.0, 20.0, -160.0,  140.0, 0.6);

        // Hazard walls
        spawn_hazard(commands,  200.0, 0.0, 20.0, 400.0);
        spawn_hazard(commands, -200.0, 0.0, 20.0, 400.0);

        // Goal — only reachable from ceiling
        spawn_goal(commands, -180.0, -170.0);
    }
}