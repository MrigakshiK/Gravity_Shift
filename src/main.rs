use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct PlayerSprite;

#[derive(Component)]
struct VisualRotation {
    current: f32,
    target: f32,
}

#[derive(Component)]
struct Hazard;

#[derive(Component)]
struct JumpCount(u8);

#[derive(Resource, Default)]
struct Deaths(u32);

#[derive(Component)]
struct DeathText;

#[derive(Component)]
struct MenuItem;

#[derive(Component)]
struct GameOverItem;

const SPAWN_POS: Vec3 = Vec3::new(0.0, 150.0, 0.0);
const DEATH_Y: f32 = 400.0;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    MainMenu,
    Playing,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec2::NEG_Y * 1800.0))
        .init_resource::<Deaths>()
        .init_state::<GameState>()  // 👈
        .add_systems(Startup, setup_hud)
        .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
        .add_systems(OnEnter(GameState::Playing), setup)
        .add_systems(OnEnter(GameState::GameOver), setup_game_over)
        .add_systems(OnExit(GameState::MainMenu), cleanup::<MenuItem>)
        .add_systems(OnExit(GameState::Playing), cleanup::<Player>)
        .add_systems(OnExit(GameState::GameOver), cleanup::<GameOverItem>)
        .add_systems(
            Update,
            (move_player, respawn_player, flip_gravity, hazard_death, update_hud, animate_rotation)
                .run_if(in_state(GameState::Playing)),  // 👈 only run while playing
        )
        .add_systems(
            Update,
            main_menu_input.run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(
            Update,
            game_over_input.run_if(in_state(GameState::GameOver)),
        )
        .run();
}

fn setup_hud(mut commands: Commands) {
    commands.spawn((
        DeathText,
        Text::new("Deaths: 0"),
        TextFont { font_size: 24.0, ..default() },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        },
    ));
}

fn cleanup<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_main_menu(mut commands: Commands) {
    commands.spawn((
        MenuItem,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(16.0),
            ..default()
        },
    )).with_children(|parent| {
        parent.spawn((
            Text::new("GRAVITY SHIFT"),
            TextFont { font_size: 64.0, ..default() },
            TextColor(Color::srgb(0.5, 0.4, 0.9)),
        ));
        parent.spawn((
            Text::new("Press SPACE to Play"),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::WHITE),
        ));
        parent.spawn((
            Text::new("A/D or Arrow Keys — Move    W or Up — Jump    Space — Flip Gravity"),
            TextFont { font_size: 14.0, ..default() },
            TextColor(Color::srgb(0.6, 0.6, 0.6)),
        ));
    });
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Physics body — no sprite, no rotation
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
        // Visual child — only this rotates
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

    spawn_platform(&mut commands, 0.0, -200.0, 600.0, 20.0);
    spawn_platform(&mut commands, 0.0, 200.0, 600.0, 20.0);
    spawn_platform(&mut commands, -180.0, 0.0, 150.0, 20.0);
    spawn_platform(&mut commands, 100.0, 60.0, 120.0, 20.0);
    spawn_platform(&mut commands, -50.0, -80.0, 100.0, 20.0);

    spawn_hazard(&mut commands, 0.0, -180.0, 60.0, 20.0);
    spawn_hazard(&mut commands, -180.0, 20.0, 60.0, 20.0);
    spawn_hazard(&mut commands, 100.0, 80.0, 60.0, 20.0);
}

fn spawn_platform(commands: &mut Commands, x: f32, y: f32, w: f32, h: f32) {
    commands.spawn((
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

fn spawn_hazard(commands: &mut Commands, x: f32, y: f32, w: f32, h: f32) {
    commands.spawn((
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

fn update_hud(
    deaths: Res<Deaths>,
    mut query: Query<&mut Text, With<DeathText>>,
) {
    if deaths.is_changed() {
        let mut text = query.single_mut();
        **text = format!("Deaths: {}", deaths.0);
    }
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut LinearVelocity, &ShapeHits, &mut JumpCount, &mut VisualRotation), With<Player>>,
    gravity: Res<Gravity>,
) {
    let (mut velocity, ground_hits, mut jump_count, mut rotation) = query.single_mut();
    let speed = 250.0;
    let jump_force = 700.0;

    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        velocity.x = -speed;
    } else if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        velocity.x = speed;
    } else {
        velocity.x = 0.0;
    }

    // 👇 snap exactly 90° on each keypress, not held
    if keys.just_pressed(KeyCode::KeyA) || keys.just_pressed(KeyCode::ArrowLeft) {
        rotation.target += std::f32::consts::FRAC_PI_2;
    }
    if keys.just_pressed(KeyCode::KeyD) || keys.just_pressed(KeyCode::ArrowRight) {
        rotation.target -= std::f32::consts::FRAC_PI_2;
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

fn flip_gravity(
    keys: Res<ButtonInput<KeyCode>>,
    mut gravity: ResMut<Gravity>,
    mut caster_query: Query<&mut ShapeCaster, With<Player>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        gravity.0 = -gravity.0;
        let mut caster = caster_query.single_mut();
        caster.direction = if gravity.0.y < 0.0 { Dir2::NEG_Y } else { Dir2::Y };
    }
}

fn respawn_player(
    mut query: Query<(&mut Transform, &mut LinearVelocity, &mut AngularVelocity, &mut JumpCount, &mut VisualRotation), With<Player>>,
    mut deaths: ResMut<Deaths>,
) {
    let (mut transform, mut linear_vel, mut angular_vel, mut jump_count, mut rotation) = query.single_mut();

    if transform.translation.y.abs() > DEATH_Y {
        transform.translation = SPAWN_POS;
        linear_vel.0 = Vec2::ZERO;
        angular_vel.0 = 0.0;
        jump_count.0 = 2;
        rotation.current = 0.0;
        rotation.target = 0.0;
        deaths.0 += 1;
    }
}

fn hazard_death(
    mut player_query: Query<(Entity, &mut Transform, &mut LinearVelocity, &mut AngularVelocity, &mut JumpCount, &mut VisualRotation), With<Player>>,
    hazard_query: Query<Entity, With<Hazard>>,
    collisions: Res<Collisions>,
    mut deaths: ResMut<Deaths>,
) {
    let (player_entity, mut transform, mut linear_vel, mut angular_vel, mut jump_count, mut rotation) = player_query.single_mut();

    for hazard_entity in &hazard_query {
        if collisions.contains(player_entity, hazard_entity) {
            transform.translation = SPAWN_POS;
            linear_vel.0 = Vec2::ZERO;
            angular_vel.0 = 0.0;
            jump_count.0 = 2;
            rotation.current = 0.0;
            rotation.target = 0.0;
            deaths.0 += 1;
        }
    }
}

fn animate_rotation(
    time: Res<Time>,
    mut player_query: Query<(&mut VisualRotation, &Children), With<Player>>,
    mut sprite_query: Query<&mut Transform, With<PlayerSprite>>,
) {
    let Ok((mut rotation, children)) = player_query.get_single_mut() else { return };

    let speed = 14.0; // lower = slower/smoother, higher = snappier
    rotation.current = rotation.current
        + (rotation.target - rotation.current) * speed * time.delta_secs();

    for child in children {
        if let Ok(mut transform) = sprite_query.get_mut(*child) {
            transform.rotation = Quat::from_rotation_z(rotation.current);
        }
    }
}