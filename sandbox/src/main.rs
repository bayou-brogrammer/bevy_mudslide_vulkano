#![allow(clippy::too_many_arguments)]

mod bevy_config;
mod fs_interaction;
mod gui;
mod input;
mod matter;
mod render;
mod settings;
mod simulator;
mod time;
mod utils;

use bevy::{ecs::schedule::LogLevel, prelude::*, window::close_on_esc};

pub const WORLD_UNIT_SIZE: f32 = 10.0;
pub const SIM_CANVAS_SIZE: u32 = 512;

/// Kernel size x & y
pub const KERNEL_SIZE: u32 = 32;
pub const NUM_WORK_GROUPS: u32 = SIM_CANVAS_SIZE / KERNEL_SIZE;

pub const CLEAR_COLOR: [f32; 4] = [0.05; 4];
pub const CAMERA_MOVE_SPEED: f32 = 200.0;
pub const SIM_FPS: f32 = 60.0;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash, Reflect)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // The simulation is running
    Simulating,
}

fn main() {
    App::new()
        .edit_schedule(bevy::app::CoreSchedule::Main, |schedule| {
            schedule.set_build_settings(bevy::ecs::schedule::ScheduleBuildSettings {
                ambiguity_detection: LogLevel::Warn,
                ..default()
            });
        })
        .add_state::<GameState>()
        .add_plugin(bevy_config::BevyConfigPlugin)
        .add_plugin(settings::SandSettingsPlugin)
        .add_plugin(fs_interaction::FileSystemPlugin)
        .add_plugin(gui::GuiPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(render::RenderPlugin)
        .add_plugin(simulator::SimulatorPlugin)
        .add_plugin(time::TimerPlugin)
        .add_system(close_on_esc)
        .run();
}
