pub mod draw_state;
pub mod painter;
pub mod windows;

use std::collections::BTreeMap;

use bevy::{ecs::system::SystemState, prelude::*, window::PrimaryWindow};
use bevy_fn_plugin::bevy_plugin;
use bevy_vulkano::{
    egui_winit_vulkano::{egui::TextureId, Gui},
    BevyVulkanoWindows,
};
use vulkano::format::Format;

use crate::{matter::matter_definition::MatterDefinitions, utils::AppExt, GameState};

pub const IMAGE_FORMAT: Format = Format::R8G8B8A8_UNORM;

#[bevy_plugin]
pub fn EditorPlugin(app: &mut App) {
    app.init_resource_on_enter::<_, Editor>(GameState::Simulating)
        .add_plugin(windows::EditorWindowsPlugin);
}

#[derive(Resource)]
pub struct Editor {
    pub show_info_view: bool,
    pub show_edit_view: bool,
    // pub show_load_view: bool,
    // pub show_guide_view: bool,
    pub show_settings_view: bool,
    // pub show_new_matter_view: bool,
    // add_matter: MatterDefinition,
    pub matter_texture_ids: BTreeMap<u32, TextureId>,
}

impl FromWorld for Editor {
    fn from_world(world: &mut World) -> Self {
        #[allow(clippy::type_complexity)]
        let mut system_state: SystemState<(
            Res<MatterDefinitions>,
            NonSendMut<BevyVulkanoWindows>,
            Query<Entity, With<PrimaryWindow>>,
        )> = SystemState::new(world);
        let (matter_definitions, mut vulkano_windows, window_query) = system_state.get_mut(world);

        let window_entity = window_query.single();
        let primary_window = vulkano_windows
            .get_vulkano_window_mut(window_entity)
            .unwrap();

        let mut editor = Self::new();
        editor.register_gui_images(&mut primary_window.gui, &matter_definitions);
        editor
    }
}

impl Editor {
    pub fn new() -> Self {
        Self {
            show_edit_view: true,
            show_info_view: false,
            show_settings_view: false,
            matter_texture_ids: BTreeMap::new(),
        }
    }

    fn register_matter_gui_images(
        &mut self,
        gui: &mut Gui,
        matter_definitions: &MatterDefinitions,
    ) {
        let material_texture_dimensions = (24, 24);

        matter_definitions.definitions.iter().for_each(|matter| {
            let image_byte_data =
                crate::utils::gui_texture_rgba_data(matter, material_texture_dimensions);
            let texture_id = gui.register_user_image_from_bytes(
                &image_byte_data,
                [material_texture_dimensions.0, material_texture_dimensions.1],
                IMAGE_FORMAT,
            );
            self.matter_texture_ids.insert(matter.id, texture_id);
        });
    }

    pub fn register_gui_images(&mut self, gui: &mut Gui, matter_definitions: &MatterDefinitions) {
        self.register_matter_gui_images(gui, matter_definitions);
    }

    pub fn update_matter_gui_textures(
        &mut self,
        gui: &mut Gui,
        matter_definitions: &MatterDefinitions,
    ) {
        self.matter_texture_ids
            .iter()
            .for_each(|(_key, texture)| gui.unregister_user_image(*texture));
        self.register_matter_gui_images(gui, matter_definitions);
    }
}
