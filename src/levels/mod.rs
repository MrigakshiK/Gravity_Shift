pub mod level1;

use avian2d::prelude::*;
use bevy::prelude::*;
use crate::components::{Hazard, LevelEntity};

pub trait Level {
    fn spawn(&self, commands: &mut Commands);
}

pub fn spawn_platform(commands: &mut Commands, x: f32, y: f32, w: f32, h: f32) {
    commands.spawn((
        LevelEntity,
        Sprite {
            color: Color::srgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(w, h)),
            ..default()
        },
        Transform::from_xyz(x, y, 0.0),
        RigidBody::Static,
        Collider::rectangle(w, h),
    ));
}

pub fn spawn_hazard(commands: &mut Commands, x: f32, y: f32, w: f32, h: f32) {
    commands.spawn((
        LevelEntity,
        Hazard,
        Sprite {
            color: Color::srgb(0.9, 0.2, 0.2),
            custom_size: Some(Vec2::new(w, h)),
            ..default()
        },
        Transform::from_xyz(x, y, 0.0),
        RigidBody::Static,
        Collider::rectangle(w, h),
        Sensor,
    ));
}