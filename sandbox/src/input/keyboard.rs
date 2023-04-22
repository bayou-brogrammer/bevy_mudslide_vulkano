use crate::{settings::AppSettings, GameState};
use bevy::prelude::*;
use bevy_fn_plugin::bevy_plugin;

#[bevy_plugin]
pub fn KeyboardPlugin(app: &mut App) {
    app.add_systems((keyboard_input_system,).distributive_run_if(in_state(GameState::Simulating)));
}

pub fn keyboard_input_system(
    mut settings: ResMut<AppSettings>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        settings.is_paused = !settings.is_paused;
    }
}
