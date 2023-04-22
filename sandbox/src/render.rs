pub mod camera;
pub mod fill_render_pass;
pub mod quad_pipeline;
pub mod utils;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_fn_plugin::bevy_plugin;
use bevy_vulkano::{BevyVulkanoContext, BevyVulkanoWindows};

use self::{camera::OrthographicCamera, fill_render_pass::FillScreenRenderPass};
use crate::{simulator::simulation::Simulation, time::RenderTimer, GameState, CLEAR_COLOR};

#[bevy_plugin]
pub fn RenderPlugin(app: &mut App) {
    app.add_plugin(camera::CameraPlugin)
        .add_system(create_render_pass.in_schedule(OnEnter(GameState::Simulating)))
        .add_system(
            render_pass
                .run_if(in_state(GameState::Simulating))
                .in_base_set(CoreSet::PostUpdate),
        );
}

fn create_render_pass(
    mut commands: Commands,
    context: Res<BevyVulkanoContext>,
    windows: NonSend<BevyVulkanoWindows>,
    window_query: Query<Entity, With<PrimaryWindow>>,
) {
    let Some(primary_window) = crate::utils::get_primary_window(&window_query, &windows) else{return};

    // Create our render pass
    let fill_screen = FillScreenRenderPass::new(
        context.context.memory_allocator().clone(),
        primary_window.renderer.graphics_queue(),
        primary_window.renderer.swapchain_format(),
    );

    commands.insert_resource(fill_screen);
}

fn render_pass(
    simulator: Res<Simulation>,
    camera: Res<OrthographicCamera>,
    mut render_timer: ResMut<RenderTimer>,
    mut fill_screen: ResMut<FillScreenRenderPass>,
    window_query: Query<Entity, With<PrimaryWindow>>,
    mut vulkano_windows: NonSendMut<BevyVulkanoWindows>,
) {
    render_timer.0.start();
    let Some(primary_window) = crate::utils::get_primary_window_mut(&window_query, &mut vulkano_windows) else{return};

    // Start frame
    let before = match primary_window.renderer.acquire() {
        Err(e) => {
            bevy::log::error!("Failed to start frame: {}", e);
            return;
        }
        Ok(f) => f,
    };

    let canvas_image = simulator.canvas_image();

    // Render
    let final_image = primary_window.renderer.swapchain_image_view();
    let after_images = fill_screen.draw(
        before,
        *camera,
        canvas_image,
        final_image.clone(),
        CLEAR_COLOR,
        false,
        true,
    );

    // Draw gui
    let after_gui = primary_window.gui.draw_on_image(after_images, final_image);
    // Finish Frame
    primary_window.renderer.present(after_gui, true);
    render_timer.0.time_it();
}
