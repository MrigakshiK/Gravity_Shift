use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

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

#[derive(Resource, Default)]
pub struct CurrentLevel(pub usize);  // 0-indexed

#[derive(Component)]
pub struct LevelSelectItem;

#[derive(Component)]
pub struct GameOverItem;

#[derive(Component)]
pub struct LevelEntity;

#[derive(Component)]
pub struct GoalTile;

#[derive(Component)]
pub struct MovingPlatform {
    pub start: Vec2,
    pub end: Vec2,
    pub speed: f32,
    pub t: f32,          // 0.0 → 1.0, position along the path
    pub forward: bool,   // direction of travel
}

pub const SPAWN_POS: Vec3 = Vec3::new(0.0, 150.0, 0.0);
pub const DEATH_Y: f32 = 400.0;