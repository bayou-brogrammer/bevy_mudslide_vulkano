pub mod ca_simulator;
pub mod gpu_utils;
pub mod simulation;

use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer, window::PrimaryWindow};
use bevy_fn_plugin::bevy_plugin;
use bevy_vulkano::{BevyVulkanoContext, BevyVulkanoWindows};

use self::simulation::Simulation;
use crate::{
    matter::matter_definition::MatterDefinitions, settings::AppSettings, time::SimulationTimer,
    GameState,
};

pub const TIME_STEP: f32 = 1.0 / 60.0;

#[bevy_plugin]
pub fn SimulatorPlugin(app: &mut App) {
    app.add_system(setup_simulation.in_schedule(OnEnter(GameState::Simulating)))
        .add_system(
            run_simulation
                .run_if(in_state(GameState::Simulating))
                .run_if(on_timer(Duration::from_secs_f32(TIME_STEP))),
        );
}

fn setup_simulation(
    mut commands: Commands,
    context: Res<BevyVulkanoContext>,
    windows: NonSend<BevyVulkanoWindows>,
    matter_definitions: Res<MatterDefinitions>,
    window_query: Query<Entity, With<PrimaryWindow>>,
) {
    let Some(primary_window) = crate::utils::get_primary_window(&window_query, &windows) else{return};

    let sim = Simulation::new(
        context.context.memory_allocator(),
        primary_window.renderer.graphics_queue(),
        &matter_definitions,
    )
    .unwrap();

    commands.insert_resource(sim);
}

fn run_simulation(
    settings: Res<AppSettings>,
    mut simulation: ResMut<Simulation>,
    mut sim_timer: ResMut<SimulationTimer>,
) {
    sim_timer.0.start();
    simulation.step(&settings);
    sim_timer.0.time_it();
}
