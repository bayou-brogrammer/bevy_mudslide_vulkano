use bevy::{prelude::*, window::PrimaryWindow};
use bevy_vulkano::{egui_winit_vulkano::egui, BevyVulkanoWindows};

use crate::gui::editor::Editor;

pub fn top_editor(
    mut state: ResMut<Editor>,
    vulkan_windows: NonSend<BevyVulkanoWindows>,
    window_query: Query<Entity, With<PrimaryWindow>>,
) {
    let Some(primary_window) = crate::utils::get_primary_window(&window_query, &vulkan_windows) else { return; };
    let ctx = primary_window.gui.context();

    egui::TopBottomPanel::top("Test").show(&ctx, |ui| {
        ui.horizontal(|ui| {
            ui.selectable_label(state.show_edit_view, "Editor")
                .clicked()
                .then(|| {
                    state.show_edit_view = !state.show_edit_view;
                });

            ui.selectable_label(state.show_settings_view, "Settings")
                .clicked()
                .then(|| {
                    state.show_settings_view = !state.show_settings_view;
                });

            ui.selectable_label(state.show_info_view, "Info")
                .clicked()
                .then(|| {
                    state.show_info_view = !state.show_info_view;
                });
        });
    });
}
