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
        VisualRotation { current: 0.0, target: 0.0 },
        Transform::from_xyz(SPAWN_POS.x, SPAWN_POS.y, SPAWN_POS.z),
        Visibility::Visible,
        RigidBody::Dynamic,
        Collider::rectangle(40.0, 40.0),
        LockedAxes::ROTATION_LOCKED,
        Friction::new(0.0),
        ShapeCaster::new(
            Collider::rectangle(36.0, 4.0),
            Vec2::ZERO,
            0.0,
            Dir2::NEG_Y,
        )
        .with_max_distance(22.0),
    ))
    .with_children(|parent| {
        parent.spawn((
            PlayerSprite,
            Sprite {
                color: Color::srgb(0.5, 0.4, 0.9),
                custom_size: Some(Vec2::new(40.0, 40.0)),
                ..default()
            },
            Transform::default(),
        ));
    });
}

pub fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut LinearVelocity, &ShapeHits, &mut JumpCount, &mut VisualRotation), With<Player>>,
    gravity: Res<Gravity>,
) {
    let Ok((mut velocity, ground_hits, mut jump_count, mut rotation)) = query.get_single_mut() else { return };
    let speed = 250.0;
    let jump_force = 700.0;

    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        velocity.x = -speed;
        if !ground_hits.is_empty() {
            rotation.target += std::f32::consts::FRAC_PI_2 * 0.15 * time.delta_secs() * 60.0;
        }
    } else if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        velocity.x = speed;
        if !ground_hits.is_empty() {
            rotation.target -= std::f32::consts::FRAC_PI_2 * 0.15 * time.delta_secs() * 60.0;
        }
    } else {
        velocity.x = 0.0;
    }

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
        &mut VisualRotation,
    ), With<Player>>,
    mut deaths: ResMut<Deaths>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok((mut transform, mut vel, mut avel, mut jumps, mut rot)) = query.get_single_mut() else { return };

    if transform.translation.y.abs() > DEATH_Y {
        deaths.0 += 1;
        if deaths.0 >= 10 {
            next_state.set(GameState::GameOver);
            return;
        }
        reset_player(&mut transform, &mut vel, &mut avel, &mut jumps, &mut rot);
    }
}

pub fn hazard_death(
    mut player_query: Query<(
        Entity,
        &mut Transform,
        &mut LinearVelocity,
        &mut AngularVelocity,
        &mut JumpCount,
        &mut VisualRotation,
    ), With<Player>>,
    hazard_query: Query<Entity, With<Hazard>>,
    collisions: Res<Collisions>,
    mut deaths: ResMut<Deaths>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok((player_entity, mut transform, mut vel, mut avel, mut jumps, mut rot)) = player_query.get_single_mut() else { return };

    for hazard_entity in &hazard_query {
        if collisions.contains(player_entity, hazard_entity) {
            deaths.0 += 1;
            if deaths.0 >= 10 {
                next_state.set(GameState::GameOver);
                return;
            }
            reset_player(&mut transform, &mut vel, &mut avel, &mut jumps, &mut rot);
        }
    }
}

pub fn animate_rotation(
    time: Res<Time>,
    mut player_query: Query<(&mut VisualRotation, &Children), With<Player>>,
    mut sprite_query: Query<&mut Transform, With<PlayerSprite>>,
) {
    let Ok((mut rotation, children)) = player_query.get_single_mut() else { return };
    let speed = 18.0;

    rotation.current += (rotation.target - rotation.current) * speed * time.delta_secs();

    for child in children {
        if let Ok(mut transform) = sprite_query.get_mut(*child) {
            transform.rotation = Quat::from_rotation_z(rotation.current);
        }
    }
}

// shared reset logic used by both death systems
fn reset_player(
    transform: &mut Transform,
    vel: &mut LinearVelocity,
    avel: &mut AngularVelocity,
    jumps: &mut JumpCount,
    rot: &mut VisualRotation,
) {
    transform.translation = SPAWN_POS;
    vel.0 = Vec2::ZERO;
    avel.0 = 0.0;
    jumps.0 = 2;
    rot.current = 0.0;
    rot.target = 0.0;
}