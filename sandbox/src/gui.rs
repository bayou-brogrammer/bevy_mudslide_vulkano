#![allow(clippy::too_many_arguments)]

pub mod editor;
pub mod painter;

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_fn_plugin::bevy_plugin;
use bevy_vulkano::{
    egui_winit_vulkano::egui::{self, Ui},
    BevyVulkanoWindows,
};

use crate::{
    render::camera::OrthographicCamera,
    simulator::simulation::Simulation,
    time::{RenderTimer, SimulationTimer},
};

use self::editor::painter::EditorPainter;

#[bevy_plugin]
pub fn GuiPlugin(app: &mut App) {
    app.add_plugin(editor::EditorPlugin)
        .add_plugin(painter::PainterPlugin)
        .add_system(user_interface.run_if(in_state(crate::GameState::Simulating)));
}

// Give our text a custom size
fn sized_text(ui: &mut Ui, text: impl Into<String>, size: f32) {
    ui.label(egui::RichText::new(text).size(size));
}

/// System to generate user interface with egui
pub fn user_interface(
    diagnostics: Res<Diagnostics>,
    render_timer: Res<RenderTimer>,
    camera: Res<OrthographicCamera>,
    sim_timer: Res<SimulationTimer>,

    mut simulator: ResMut<Simulation>,
    mut painter: ResMut<EditorPainter>,

    window_query: Query<(Entity, &Window)>,
    vulkan_windows: NonSend<BevyVulkanoWindows>,
) {
    let (window_entity, window) = window_query.single();
    let primary_window = vulkan_windows.get_vulkano_window(window_entity).unwrap();
    let ctx = primary_window.gui.context();

    egui::Area::new("fps")
        .fixed_pos(egui::pos2(10.0, 10.0))
        .show(&ctx, |ui| {
            let size = 15.0;
            ui.heading("Info");

            if let Some(diag) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
                if let Some(avg) = diag.average() {
                    sized_text(ui, format!("FPS: {:.2}", avg), size);
                }
            }

            sized_text(
                ui,
                format!("Sim Time: {:.2} ms", sim_timer.0.time_average_ms(),),
                size,
            );

            sized_text(
                ui,
                format!("Render Time: {:.2} ms", render_timer.0.time_average_ms(),),
                size,
            );
        });
}
