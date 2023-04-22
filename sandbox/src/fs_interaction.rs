pub mod asset_loading;
pub mod config;
pub mod file_utils;
use bevy_fn_plugin::bevy_plugin;
pub use file_utils::*;

use crate::fs_interaction::asset_loading::LoadingPlugin;

/// Handles loading and saving of levels and save states to disk.
/// Split into the following sub-plugins:
/// - [`loading_plugin`] handles loading of assets.
#[bevy_plugin]
pub fn FileSystemPlugin(app: &mut App) {
    app.add_plugin(LoadingPlugin);
}
