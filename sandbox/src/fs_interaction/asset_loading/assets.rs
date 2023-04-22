use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::fs_interaction::config::GameConfig;

#[derive(AssetCollection, Resource, Clone)]
pub struct SimulationAssets {
    #[asset(path = "config.game.toml")]
    pub game: Handle<GameConfig>,
}
