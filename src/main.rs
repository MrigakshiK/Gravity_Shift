mod components;
mod levels;
mod player;
mod state;
mod systems;
mod ui;

use avian2d::prelude::*;
use bevy::prelude::*;
use components::*;
use state::GameState;
use systems::cleanup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec2::NEG_Y * 1800.0))
        .init_resource::<Deaths>()
        .init_state::<GameState>()
        // always present
        .add_systems(Startup, (ui::setup_camera, ui::setup_hud))
        // state enter
        .add_systems(OnEnter(GameState::MainMenu), ui::setup_main_menu)
        .add_systems(OnEnter(GameState::Playing),  player::setup_player)
        .add_systems(OnEnter(GameState::GameOver),  ui::setup_game_over)
        // state exit — cleanup everything tagged for that state
        .add_systems(OnExit(GameState::MainMenu), cleanup::<MenuItem>)
        .add_systems(OnExit(GameState::Playing),  (cleanup::<Player>, cleanup::<LevelEntity>))
        .add_systems(OnExit(GameState::GameOver),  cleanup::<GameOverItem>)
        // gameplay systems — only run while playing
        .add_systems(
            Update,
            (
                player::move_player,
                player::respawn_player,
                player::flip_gravity,
                player::hazard_death,
                ui::update_hud,
            )
            .run_if(in_state(GameState::Playing)),
        )
        // menu input
        .add_systems(Update, ui::main_menu_input.run_if(in_state(GameState::MainMenu)))
        .add_systems(Update, ui::game_over_input.run_if(in_state(GameState::GameOver)))
        .run();
}