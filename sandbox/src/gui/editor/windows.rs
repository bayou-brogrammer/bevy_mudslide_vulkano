pub mod editor_window;
pub mod info_window;
pub mod top_editor;

use bevy::prelude::*;
use bevy_fn_plugin::bevy_plugin;

use crate::GameState;

#[bevy_plugin]
pub fn EditorWindowsPlugin(app: &mut App) {
    app.add_systems(
        (
            top_editor::top_editor,
            info_window::info_window,
            editor_window::editor_window,
        )
            .distributive_run_if(in_state(GameState::Simulating)),
    );
}
