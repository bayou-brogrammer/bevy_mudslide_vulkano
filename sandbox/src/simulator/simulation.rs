use std::sync::Arc;

use anyhow::Result;
use bevy::prelude::*;
use vulkano::{device::Queue, memory::allocator::StandardMemoryAllocator};
use vulkano_util::renderer::DeviceImageView;

use super::ca_simulator::CASimulator;
use crate::{
    matter::matter_definition::MatterDefinitions, settings::AppSettings,
    time::performance_timer::PerformanceTimer,
};

#[derive(Resource)]
pub struct Simulation {
    ca_simulator: CASimulator,
    pub ca_timer: PerformanceTimer,
}

impl Simulation {
    pub fn new(
        allocator: &Arc<StandardMemoryAllocator>,
        compute_queue: Arc<Queue>,
        matter_definitions: &MatterDefinitions,
    ) -> Result<Simulation> {
        let mut ca_simulator = CASimulator::new(allocator, compute_queue, matter_definitions)?;
        ca_simulator.update_matter_data(matter_definitions)?;

        Ok(Simulation {
            ca_simulator,
            ca_timer: PerformanceTimer::default(),
        })
    }

    pub fn canvas_image(&self) -> DeviceImageView {
        self.ca_simulator.color_image()
    }

    pub fn step(&mut self, settings: &AppSettings) {
        self.ca_timer.start();
        self.ca_simulator.step(settings);
        self.ca_timer.time_it();
    }

    pub fn paint_round(
        &mut self,
        start: Vec2,
        end: Vec2,
        matter: u32,
        radius: f32,
        is_square: bool,
    ) -> Result<()> {
        self.ca_simulator
            .draw_matter(start, end, matter, radius, is_square);
        Ok(())
    }

    pub fn query_matter(&mut self, pos: IVec2) -> Option<u32> {
        self.ca_simulator.query_matter(pos)
    }
}
