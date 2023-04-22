pub mod keyboard;
pub mod mouse;

use bevy::{
    input::{mouse::MouseButtonInput, ButtonState},
    prelude::*,
};
use bevy_fn_plugin::bevy_plugin;
use bevy_vulkano::BevyVulkanoWindows;

use crate::{render::camera::OrthographicCamera, GameState};

#[bevy_plugin]
pub fn InputPlugin(app: &mut App) {
    app.init_resource::<InputState>()
        .add_plugin(mouse::MousePlugin)
        .add_plugin(keyboard::KeyboardPlugin)
        .add_system(update_input_state.run_if(in_state(GameState::Simulating)));
}

#[derive(Default, Resource)]
pub struct InputState {
    can_scroll: bool,
    mouse_world_pos: Vec2,
    mouse_canvas_pos: Vec2,
    left_button_down: bool,
}

impl InputState {
    pub fn mouse_canvas_pos(&self) -> Vec2 {
        self.mouse_canvas_pos
    }
}

pub fn update_input_state(
    window_query: Query<(Entity, &Window)>,
    camera: Res<OrthographicCamera>,
    mut input_state: ResMut<InputState>,
    vulkano_windows: NonSend<BevyVulkanoWindows>,
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
) {
    let Ok((window_entity, primary)) = window_query.get_single() else { return };
    let Some(vulkan_window) = vulkano_windows.get_vulkano_window(window_entity) else { return };

    let ctx = vulkan_window.gui.context();
    if ctx.wants_pointer_input()
        || ctx.is_pointer_over_area()
        || ctx.is_using_pointer()
        || ctx.wants_pointer_input()
    {
        // GUI gets priority input
        input_state.left_button_down = false;
        input_state.can_scroll = false;
        return;
    } else {
        input_state.can_scroll = true;
    }

    // Determine button state
    for event in mouse_button_input_events.iter() {
        if event.button == MouseButton::Left {
            input_state.left_button_down = event.state == ButtonState::Pressed;
        }
    }

    // Update Mouse Position
    if let Some(cursor_pos) = primary.cursor_position() {
        input_state.mouse_world_pos = camera.screen_to_world_pos(primary, cursor_pos);
        input_state.mouse_canvas_pos =
            crate::utils::world_pos_to_canvas_pos(input_state.mouse_world_pos);
    }
}
