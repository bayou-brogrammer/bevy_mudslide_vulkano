use bevy::prelude::*;
use bevy_fn_plugin::bevy_plugin;
use bevy_vulkano::BevyVulkanoContext;
use vulkano::device::physical::PhysicalDeviceType;

use crate::{utils::AppExt, GameState};

#[bevy_plugin]
//noinspection RsFunctionNaming
pub fn SandSettingsPlugin(app: &mut App) {
    app.init_resource_on_exit::<_, DeviceProperties>(GameState::Loading)
        .init_resource_on_enter::<_, AppSettings>(GameState::Simulating);
}

pub const INIT_MOVEMENT_STEPS: u32 = 3;
pub const INIT_DISPERSION_STEPS: u32 = 10;

#[derive(Debug, Clone, Copy, Resource)]
pub struct AppSettings {
    pub is_paused: bool,
    pub movement_steps: u32,
    pub dispersion_steps: u32,
    pub print_performance: bool,
}

impl FromWorld for AppSettings {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let mut settings = Self::new();
        let properties = world.get_resource::<DeviceProperties>().unwrap();
        settings.update_based_on_device_info_and_env(properties);
        settings
    }
}

impl AppSettings {
    pub fn new() -> AppSettings {
        let dispersion_steps = INIT_DISPERSION_STEPS;
        let movement_steps = INIT_MOVEMENT_STEPS;
        AppSettings {
            movement_steps,
            is_paused: false,
            dispersion_steps,
            print_performance: false,
        }
    }

    pub fn update_based_on_device_info_and_env(&mut self, properties: &DeviceProperties) {
        let max_mem_gb = properties.max_mem_gb();
        let device_type = properties.device_type();
        if device_type != PhysicalDeviceType::DiscreteGpu {
            log::info!("Reduce default settings (No discrete gpu)");
            self.dispersion_steps = 4;
            self.movement_steps = 1;
        } else if max_mem_gb < 2.0 {
            log::info!("Reduce default settings (< 2.0 gb gpu mem)");
            self.dispersion_steps = 4;
            self.movement_steps = 2;
        } else if max_mem_gb < 1.0 {
            log::info!("Reduce default settings (< 1.0 gb gpu mem)");
            self.dispersion_steps = 3;
            self.movement_steps = 1;
        };
    }
}

#[derive(Debug, Resource)]
pub struct DeviceProperties {
    max_mem_gb: f32,
    #[allow(unused)]
    device_name: String,
    device_type: PhysicalDeviceType,
}

impl DeviceProperties {
    pub fn device_type(&self) -> PhysicalDeviceType {
        self.device_type
    }

    pub fn max_mem_gb(&self) -> f32 {
        self.max_mem_gb
    }
}

impl FromWorld for DeviceProperties {
    fn from_world(world: &mut bevy::prelude::World) -> Self {
        let ctx = &world.get_resource::<BevyVulkanoContext>().unwrap().context;
        let physical_device = ctx.device().physical_device();

        // Get desired device
        let device_type = physical_device.properties().device_type;
        // Get device name
        let device_name = physical_device.properties().device_name.to_string();

        #[cfg(target_os = "windows")]
        let max_mem_gb = physical_device.properties().max_memory_allocation_count as f32 * 9.31e-4;
        #[cfg(not(target_os = "windows"))]
        let max_mem_gb = physical_device.properties().max_memory_allocation_count as f32 * 9.31e-10;

        log::info!(
            "Using device {}, type: {:?}, mem: {:.2} gb",
            device_name,
            device_type,
            max_mem_gb,
        );

        Self {
            max_mem_gb,
            device_name,
            device_type,
        }
    }
}
