use crate::{matter::Matters, simulator::simulation::Simulation, GameState};
use bevy::{prelude::*, window::PrimaryWindow};
use bevy_fn_plugin::bevy_plugin;
use bevy_vulkano::{egui_winit_vulkano::egui, BevyVulkanoWindows};

use super::InputState;

#[bevy_plugin]
pub fn MousePlugin(app: &mut App) {
    app.add_systems((mouse_input_system,).distributive_run_if(in_state(GameState::Simulating)));
}

pub fn mouse_input_system(
    matters: Res<Matters>,
    input_state: Res<InputState>,
    mut simulation: ResMut<Simulation>,
    vulkan_windows: NonSend<BevyVulkanoWindows>,
    window_query: Query<Entity, With<PrimaryWindow>>,
) {
    let Some(vulkan_window) = crate::utils::get_primary_window(&window_query, &vulkan_windows) else { return };
    let ctx = vulkan_window.gui.context();

    let InputState {
        mouse_world_pos,
        mouse_canvas_pos,
        ..
    } = &*input_state;

    egui::containers::show_tooltip_at_pointer(&ctx, egui::Id::new("Hover tooltip"), |ui| {
        ui.label(format!(
            "World: [{:.2}, {:.2}]",
            mouse_world_pos.x, mouse_world_pos.y
        ));
        ui.label(format!(
            "Sim: [{:.2}, {:.2}]",
            mouse_canvas_pos.x, mouse_canvas_pos.y
        ));

        if let Some(matter) = simulation.query_matter(mouse_canvas_pos.as_ivec2()) {
            ui.label(format!(
                "Matter: {}",
                matters.0.get(&matter).unwrap_or(&"Unknown".to_string())
            ));
        }
    });
}
