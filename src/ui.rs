use bevy::prelude::*;
use crate::components::*;
use crate::state::GameState;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn setup_hud(mut commands: Commands) {
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

pub fn update_hud(
    deaths: Res<Deaths>,
    mut query: Query<&mut Text, With<DeathText>>,
) {
    if deaths.is_changed() {
        if let Ok(mut text) = query.get_single_mut() {
            **text = format!("Deaths: {}", deaths.0);
        }
    }
}

pub fn setup_main_menu(mut commands: Commands) {
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
    ))
    .with_children(|parent| {
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
            Text::new("A / D — Move     W — Jump     Space — Flip Gravity"),
            TextFont { font_size: 14.0, ..default() },
            TextColor(Color::srgb(0.6, 0.6, 0.6)),
        ));
    });
}

pub fn main_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut deaths: ResMut<Deaths>,
) {
    if keys.just_pressed(KeyCode::Space) {
        deaths.0 = 0;
        next_state.set(GameState::Playing);
    }
}

pub fn setup_game_over(mut commands: Commands, deaths: Res<Deaths>) {
    commands.spawn((
        GameOverItem,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(16.0),
            ..default()
        },
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("GAME OVER"),
            TextFont { font_size: 64.0, ..default() },
            TextColor(Color::srgb(0.9, 0.2, 0.2)),
        ));
        parent.spawn((
            Text::new(format!("Deaths: {}", deaths.0)),
            TextFont { font_size: 32.0, ..default() },
            TextColor(Color::WHITE),
        ));
        parent.spawn((
            Text::new("R — Retry     M — Main Menu"),
            TextFont { font_size: 20.0, ..default() },
            TextColor(Color::srgb(0.6, 0.6, 0.6)),
        ));
    });
}

pub fn game_over_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut deaths: ResMut<Deaths>,
) {
    if keys.just_pressed(KeyCode::KeyR) {
        deaths.0 = 0;
        next_state.set(GameState::Playing);
    }
    if keys.just_pressed(KeyCode::KeyM) {
        next_state.set(GameState::MainMenu);
    }
}

#[derive(Component)]
pub struct LevelCompleteItem;

pub fn setup_level_complete(mut commands: Commands) {
    commands.spawn((
        LevelCompleteItem,
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(16.0),
            ..default()
        },
    ))
    .with_children(|parent| {
        parent.spawn((
            Text::new("LEVEL COMPLETE"),
            TextFont { font_size: 64.0, ..default() },
            TextColor(Color::srgb(0.2, 0.9, 0.4)),
        ));
        parent.spawn((
            Text::new("Press SPACE for next level"),
            TextFont { font_size: 24.0, ..default() },
            TextColor(Color::WHITE),
        ));
    });
}

pub fn level_complete_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut deaths: ResMut<Deaths>,  // 👈
) {
    if keys.just_pressed(KeyCode::Space) {
        deaths.0 = 0;            // 👈
        next_state.set(GameState::Playing);
    }
}