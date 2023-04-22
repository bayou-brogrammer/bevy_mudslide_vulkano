pub mod assets;

use anyhow::{Context, Result};
use bevy::{prelude::*, utils::HashMap};
use bevy_asset_loader::prelude::*;
use bevy_common_assets::{ron::RonAssetPlugin, toml::TomlAssetPlugin};
use bevy_fn_plugin::bevy_plugin;
use bevy_mod_sysfail::macros::*;
use iyes_progress::{Progress, ProgressPlugin};

use crate::{
    fs_interaction::config::GameConfig,
    matter::{
        matter_definition::{validate_matter_definitions, MatterDefinitions},
        Matters, MATTER_DEFINITION_FILE,
    },
    GameState,
};

use self::assets::SimulationAssets;

#[bevy_plugin]
pub fn LoadingPlugin(app: &mut App) {
    use iyes_progress::ProgressSystem;

    app.add_plugin(TomlAssetPlugin::<GameConfig>::new(&["game.toml"]))
        .add_plugin(RonAssetPlugin::<MatterDefinitions>::new(&["matter.ron"]))
        .add_plugin(ProgressPlugin::new(GameState::Loading).continue_to(GameState::Simulating))
        .add_loading_state(LoadingState::new(GameState::Loading))
        .add_collection_to_loading_state::<_, SimulationAssets>(GameState::Loading)
        // Update after loading
        .add_systems((
            update_config,
            wait_for_config
                .track_progress()
                .run_if(resource_added::<GameConfig>())
                .run_if(in_state(GameState::Loading)),
        ));
}

fn wait_for_config(mut commands: Commands, config: Res<GameConfig>) -> Progress {
    let definition_path = if let Some(path) = &config.definition_path {
        path
    } else {
        MATTER_DEFINITION_FILE
    };

    let matter_definitions =
        if let Some(defs) = crate::utils::read_matter_definitions_file(definition_path) {
            defs
        } else {
            crate::matter::default_matter_definitions()
        };

    validate_matter_definitions(&matter_definitions);

    let mut matters = HashMap::new();
    for definition in matter_definitions.definitions.iter() {
        matters.insert(definition.id, definition.name.clone());
    }

    commands.insert_resource(Matters(matters));
    commands.insert_resource(matter_definitions);

    true.into()
}

#[sysfail(log(level = "error"))]
fn update_config(
    mut commands: Commands,
    config: Res<Assets<GameConfig>>,
    mut config_asset_events: EventReader<AssetEvent<GameConfig>>,
) -> Result<()> {
    #[cfg(feature = "tracing")]
    let _span = info_span!("update_config").entered();

    for event in config_asset_events.iter() {
        match event {
            AssetEvent::Created { handle } | AssetEvent::Modified { handle } => {
                // Guaranteed by Bevy to not fail
                let config = config
                    .get(handle)
                    .context("Failed to get config even though it was just created")?;
                commands.insert_resource(config.clone());
            }
            AssetEvent::Removed { .. } => {}
        }
    }

    Ok(())
}
