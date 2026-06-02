use bevy::prelude::*;
use super::{Level, spawn_platform, spawn_hazard};

pub struct Level1;

impl Level for Level1 {
    fn spawn(&self, commands: &mut Commands) {
        // Ground + ceiling
        spawn_platform(commands,   0.0, -200.0, 600.0, 20.0);
        spawn_platform(commands,   0.0,  200.0, 600.0, 20.0);

        // Mid platforms
        spawn_platform(commands, -180.0,   0.0, 150.0, 20.0);
        spawn_platform(commands,  100.0,  60.0, 120.0, 20.0);
        spawn_platform(commands,  -50.0, -80.0, 100.0, 20.0);

        // Hazards
        spawn_hazard(commands,    0.0, -180.0, 60.0, 20.0);
        spawn_hazard(commands, -180.0,   20.0, 60.0, 20.0);
        spawn_hazard(commands,  100.0,   80.0, 60.0, 20.0);
    }
}