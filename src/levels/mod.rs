pub mod level1;

use avian2d::prelude::*;
use bevy::prelude::*;
use crate::components::{Hazard, LevelEntity, GoalTile, MovingPlatform};

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

pub fn spawn_goal(commands: &mut Commands, x: f32, y: f32) {
    commands.spawn((
        LevelEntity,
        GoalTile,
        Sprite {
            color: Color::srgb(0.2, 0.9, 0.4),
            custom_size: Some(Vec2::new(40.0, 40.0)),
            ..default()
        },
        Transform::from_xyz(x, y, 0.0),
        RigidBody::Static,
        Collider::rectangle(40.0, 40.0),
        Sensor,
    ));
}

pub fn spawn_moving_platform(
    commands: &mut Commands,
    x: f32, y: f32,
    w: f32, h: f32,
    end_x: f32, end_y: f32,
    speed: f32,
) {
    commands.spawn((
        LevelEntity,
        MovingPlatform {
            start: Vec2::new(x, y),
            end: Vec2::new(end_x, end_y),
            speed,
            t: 0.0,
            forward: true,
        },
        Sprite {
            color: Color::srgb(0.4, 0.5, 0.8),
            custom_size: Some(Vec2::new(w, h)),
            ..default()
        },
        Transform::from_xyz(x, y, 0.0),
        RigidBody::Kinematic,
        LinearVelocity::ZERO,
        Collider::rectangle(w, h),
    ));
}