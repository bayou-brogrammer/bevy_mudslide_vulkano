use bevy::prelude::*;
use bevy_fn_plugin::bevy_plugin;
use bevy_mod_sysfail::sysfail;

use super::editor::{draw_state::CanvasDrawState, painter::EditorPainter};
use crate::{input::InputState, simulator::simulation::Simulation, GameState};
use anyhow::Result;

#[bevy_plugin]
pub fn PainterPlugin(app: &mut App) {
    app.init_resource::<EditorPainter>()
        .init_resource::<CanvasDrawState>()
        .add_system(painter.run_if(in_state(GameState::Simulating)));
}

#[sysfail(log(level = "error"))]
fn painter(
    input_state: Res<InputState>,
    buttons: Res<Input<MouseButton>>,
    mut simulator: ResMut<Simulation>,
    mut painter: ResMut<EditorPainter>,
    mut draw_state: ResMut<CanvasDrawState>,
) -> Result<()> {
    if buttons.just_pressed(MouseButton::Left) {
        draw_state.start(input_state.mouse_canvas_pos());
    }
    if buttons.pressed(MouseButton::Left) {
        draw_state.draw(input_state.mouse_canvas_pos());
    }
    if buttons.just_released(MouseButton::Left) {
        draw_state.end();
    }

    if draw_state.started() {
        painter.paint_round_line(draw_state.prev, draw_state.current.unwrap(), &mut simulator)?
    }

    Ok(())
}
