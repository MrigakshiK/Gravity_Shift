use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerSprite;

#[derive(Component)]
pub struct VisualRotation {
    pub current: f32,
    pub target: f32,
}

#[derive(Component)]
pub struct Hazard;

#[derive(Component)]
pub struct JumpCount(pub u8);

#[derive(Resource, Default)]
pub struct Deaths(pub u32);

#[derive(Component)]
pub struct DeathText;

#[derive(Component)]
pub struct MenuItem;

#[derive(Component)]
pub struct GameOverItem;

#[derive(Component)]
pub struct LevelEntity;

pub const SPAWN_POS: Vec3 = Vec3::new(0.0, 150.0, 0.0);
pub const DEATH_Y: f32 = 400.0;