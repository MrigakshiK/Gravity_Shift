use avian2d::prelude::*;
use bevy::prelude::*;
use crate::components::*;
use crate::levels::level1::Level1;
use crate::levels::Level;
use crate::state::GameState;

pub fn setup_player(mut commands: Commands) {
    Level1.spawn(&mut commands);

    commands.spawn((
        Player,
        JumpCount(2),
        Sprite {
            color: Color::srgb(0.5, 0.4, 0.9),
            custom_size: Some(Vec2::new(40.0, 40.0)),
            ..default()
        },
        Transform::from_xyz(SPAWN_POS.x, SPAWN_POS.y, SPAWN_POS.z),
        RigidBody::Dynamic,
        Collider::rectangle(40.0, 40.0),
        LockedAxes::ROTATION_LOCKED,
        Friction::new(0.5), // some friction so we don't slide forever on flat ground
        ShapeCaster::new(
            Collider::rectangle(36.0, 4.0),
            Vec2::ZERO,
            0.0,
            Dir2::NEG_Y,
        )
        .with_max_distance(22.0),
    ));
}

pub fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut LinearVelocity, &ShapeHits, &mut JumpCount), With<Player>>,
    gravity: Res<Gravity>,
) {
    let Ok((mut velocity, ground_hits, mut jump_count)) = query.get_single_mut() else { return };
    let speed = 250.0;
    let jump_force = 700.0;

    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        velocity.x = -speed;
    } else if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        velocity.x = speed;
    } else if !ground_hits.is_empty() {
        // grounded + no input = decelerate, not instant stop
        // this lets platform velocity carry through
        velocity.x *= 0.75;
    }
    // in the air with no input = don't touch x at all

    if !ground_hits.is_empty() {
        jump_count.0 = 2;
    }

    if (keys.just_pressed(KeyCode::KeyW) || keys.just_pressed(KeyCode::ArrowUp)) && jump_count.0 > 0 {
        let jump_dir = if gravity.0.y < 0.0 { 1.0 } else { -1.0 };
        velocity.y = jump_force * jump_dir;
        jump_count.0 -= 1;
    }
}

pub fn flip_gravity(
    keys: Res<ButtonInput<KeyCode>>,
    mut gravity: ResMut<Gravity>,
    mut caster_query: Query<&mut ShapeCaster, With<Player>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        gravity.0 = -gravity.0;
        if let Ok(mut caster) = caster_query.get_single_mut() {
            caster.direction = if gravity.0.y < 0.0 { Dir2::NEG_Y } else { Dir2::Y };
        }
    }
}

pub fn respawn_player(
    mut query: Query<(
        &mut Transform,
        &mut LinearVelocity,
        &mut AngularVelocity,
        &mut JumpCount,
    ), With<Player>>,
    mut deaths: ResMut<Deaths>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok((mut transform, mut vel, mut avel, mut jumps)) = query.get_single_mut() else { return };

    if transform.translation.y.abs() > DEATH_Y {
        deaths.0 += 1;
        if deaths.0 >= 10 {
            next_state.set(GameState::GameOver);
            return;
        }
        reset_player(&mut transform, &mut vel, &mut avel, &mut jumps);
    }
}

pub fn hazard_death(
    mut player_query: Query<(
        Entity,
        &mut Transform,
        &mut LinearVelocity,
        &mut AngularVelocity,
        &mut JumpCount,
    ), With<Player>>,
    hazard_query: Query<Entity, With<Hazard>>,
    collisions: Res<Collisions>,
    mut deaths: ResMut<Deaths>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok((player_entity, mut transform, mut vel, mut avel, mut jumps)) = player_query.get_single_mut() else { return };

    for hazard_entity in &hazard_query {
        if collisions.contains(player_entity, hazard_entity) {
            deaths.0 += 1;
            if deaths.0 >= 10 {
                next_state.set(GameState::GameOver);
                return;
            }
            reset_player(&mut transform, &mut vel, &mut avel, &mut jumps);
        }
    }
}

// shared reset logic used by both death systems
fn reset_player(
    transform: &mut Transform,
    vel: &mut LinearVelocity,
    avel: &mut AngularVelocity,
    jumps: &mut JumpCount,
) {
    transform.translation = SPAWN_POS;
    vel.0 = Vec2::ZERO;
    avel.0 = 0.0;
    jumps.0 = 2;
}

pub fn check_goal(
    player_query: Query<Entity, With<Player>>,
    goal_query: Query<Entity, With<GoalTile>>,
    collisions: Res<Collisions>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(player) = player_query.get_single() else { return };

    for goal in &goal_query {
        if collisions.contains(player, goal) {
            next_state.set(GameState::LevelComplete);
        }
    }
}

pub fn move_platforms(
    time: Res<Time>,
    mut query: Query<(&mut MovingPlatform, &mut LinearVelocity)>,
) {
    for (mut platform, mut velocity) in &mut query {
        let prev_t = platform.t;

        if platform.forward {
            platform.t += platform.speed * time.delta_secs();
            if platform.t >= 1.0 {
                platform.t = 1.0;
                platform.forward = false;
            }
        } else {
            platform.t -= platform.speed * time.delta_secs();
            if platform.t <= 0.0 {
                platform.t = 0.0;
                platform.forward = true;
            }
        }

        let current_pos = platform.start.lerp(platform.end, platform.t);
        let prev_pos = platform.start.lerp(platform.end, prev_t);
        let delta = current_pos - prev_pos;

        // velocity = how far we moved this frame / how long the frame took
        velocity.0 = if time.delta_secs() > 0.0 {
            delta / time.delta_secs()
        } else {
            Vec2::ZERO
        };
    }
}