use bevy::{prelude::*, window::PrimaryWindow};
use bevy_vulkano::{
    egui_winit_vulkano::egui::{self, ImageButton, Ui},
    BevyVulkanoWindows,
};

use crate::{
    gui::editor::{painter::EditorPainter, Editor},
    matter::matter_definition::{MatterDefinition, MatterDefinitions},
};

pub fn editor_window(
    editor: Res<Editor>,
    mut painter: ResMut<EditorPainter>,
    matter_definitions: Res<MatterDefinitions>,
    vulkan_windows: NonSend<BevyVulkanoWindows>,
    window_query: Query<Entity, With<PrimaryWindow>>,
) {
    let Some(primary_window) = crate::utils::get_primary_window(&window_query, &vulkan_windows) else { return; };
    let ctx = primary_window.gui.context();

    let Editor {
        mut show_edit_view, ..
    } = *editor;

    egui::Window::new("Editor")
        .open(&mut show_edit_view)
        .vscroll(true)
        .default_width(200.0)
        .default_height(800.0)
        .show(&ctx, |ui| {
            ui.label("Brush Radius");
            ui.add(egui::Slider::new(painter.radius_mut(), 0.5..=30.0));

            ui.checkbox(painter.is_square_mut(), "Square brush");
            ui.separator();

            ui.label(format!(
                "Matter ({})",
                &matter_definitions.definitions[painter.get_matter() as usize].name
            ));
            ui.separator();

            add_matter_palette(ui, &editor, &mut painter, &matter_definitions);
        });
}

fn add_matter_palette(
    ui: &mut Ui,
    editor: &Editor,
    painter: &mut EditorPainter,
    matter_definitions: &MatterDefinitions,
) {
    let num_cols = 4;
    let button_size = egui::Vec2::new(24.0, 24.0);
    let grouped_matters = get_grouped_matters(&matter_definitions.definitions);

    for m_group in grouped_matters.iter() {
        let state = m_group[0].state;
        ui.label(state.to_string());
        ui.separator();

        egui::Grid::new(state.to_string()).show(ui, |ui| {
            let mut cols = 0;
            for m in m_group.iter() {
                let texture_id = editor
                    .matter_texture_ids
                    .get(&m.id)
                    .expect("Material texture id not found");

                let btn = ImageButton::new(*texture_id, button_size);

                ui.horizontal(|ui| {
                    if ui.add(btn).on_hover_text(&m.name).clicked() {
                        painter.set_matter(m.id);
                    }
                    ui.label(&m.name);
                });

                cols += 1;
                if cols == num_cols {
                    ui.end_row();
                    cols = 0;
                }
            }
        });
    }
}

fn get_grouped_matters(matters: &[MatterDefinition]) -> Vec<Vec<MatterDefinition>> {
    let mut matters: Vec<MatterDefinition> = matters.to_vec();
    matters.sort_unstable_by_key(|m| m.state);

    let mut grouped_matters = vec![];
    let mut is_next_group = true;
    let mut last_state = matters.first().unwrap().state;
    for matter in matters.iter() {
        if matter.state != last_state {
            is_next_group = true;
        }

        if is_next_group {
            grouped_matters.push(vec![matter.clone()]);
            last_state = matter.state;
            is_next_group = false;
        } else {
            grouped_matters.last_mut().unwrap().push(matter.clone());
        }
    }

    grouped_matters
}
