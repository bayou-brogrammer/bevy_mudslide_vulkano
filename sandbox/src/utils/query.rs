use bevy::{prelude::*, window::PrimaryWindow};
use bevy_vulkano::{BevyVulkanoWindows, VulkanoWindow};

use crate::{
    fs_interaction::FileUtils, matter::matter_definition::MatterDefinitions, SIM_CANVAS_SIZE,
};

pub fn get_primary_window<'a>(
    window_query: &'a Query<Entity, With<PrimaryWindow>>,
    vulkan_windows: &'a NonSend<BevyVulkanoWindows>,
) -> Option<&'a VulkanoWindow> {
    let Ok(window_entity) = window_query.get_single() else { return None };
    if let Some(vulkan_window) = vulkan_windows.get_vulkano_window(window_entity) {
        Some(vulkan_window)
    } else {
        None
    }
}

pub fn get_primary_window_mut<'a>(
    window_query: &'a Query<Entity, With<PrimaryWindow>>,
    vulkan_windows: &'a mut NonSendMut<BevyVulkanoWindows>,
) -> Option<&'a mut VulkanoWindow> {
    let Ok(window_entity) = window_query.get_single() else { return None };
    if let Some(vulkan_window) = vulkan_windows.get_vulkano_window_mut(window_entity) {
        Some(vulkan_window)
    } else {
        None
    }
}

pub fn world_pos_to_canvas_pos(world_pos: Vec2) -> Vec2 {
    world_pos + Vec2::new(SIM_CANVAS_SIZE as f32 / 2.0, SIM_CANVAS_SIZE as f32 / 2.0)
}

pub fn read_matter_definitions_file(path: &str) -> Option<MatterDefinitions> {
    match FileUtils::read_ron::<MatterDefinitions>(path) {
        Ok(matter_definitions) => Some(matter_definitions),
        Err(e) => {
            log::warn!("Error reading matter definitions: {}", e);
            None
        }
    }
}
