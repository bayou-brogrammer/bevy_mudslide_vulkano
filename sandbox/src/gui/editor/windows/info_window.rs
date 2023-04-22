use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_vulkano::{egui_winit_vulkano::egui, BevyVulkanoWindows};

use crate::{
    gui::editor::Editor,
    simulator::simulation::Simulation,
    time::{RenderTimer, SimulationTimer},
};
#[derive(Resource)]
pub struct FPSTimer(Timer);

impl Default for FPSTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

pub fn info_window(
    time: Res<Time>,
    mut fps: Local<f64>,
    editor: Res<Editor>,
    sim: Res<Simulation>,
    // mut state: Res<Editor>,
    mut timer: Local<FPSTimer>,
    diagnostics: Res<Diagnostics>,
    render_timer: Res<RenderTimer>,
    sim_timer: Res<SimulationTimer>,
    vulkan_windows: NonSend<BevyVulkanoWindows>,
    window_query: Query<Entity, With<PrimaryWindow>>,
) {
    let Some(primary_window) = crate::utils::get_primary_window(&window_query, &vulkan_windows) else { return; };
    let ctx = primary_window.gui.context();

    timer.0.tick(time.delta());

    let Editor {
        mut show_info_view, ..
    } = *editor;

    if timer.0.just_finished() {
        if let Some(diag) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(avg) = diag.average() {
                *fps = avg;
            }
        }
    }

    egui::Window::new("Info")
        .open(&mut show_info_view)
        .default_width(200.0)
        .show(&ctx, |ui| {
            ui.label("Macro level time averages:");

            ui.label(format!("FPS: {:.3}", *fps));
            // ui.label(format!("dt: {:.3}", frame_time_average));
            ui.label(format!("Render: {:.3}", render_timer.0.time_average_ms()));
            ui.label(format!("Simulation: {:.3}", sim_timer.0.time_average_ms()));

            ui.label("Breakdown");
            ui.label(format!(
                "CA simulation: {:.3}",
                sim.ca_timer.time_average_ms()
            ));
        });
}
