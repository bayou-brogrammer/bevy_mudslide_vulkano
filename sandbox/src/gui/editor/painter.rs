use anyhow::Result;
use bevy::prelude::{Resource, Vec2};

use crate::simulator::simulation::Simulation;

#[derive(Resource)]
pub struct EditorPainter {
    radius: f32,
    matter: u32,
    is_square: bool,
}

impl Default for EditorPainter {
    fn default() -> Self {
        Self {
            matter: 1,
            radius: 4.0,
            is_square: false,
        }
    }
}

impl EditorPainter {
    pub fn radius_mut(&mut self) -> &mut f32 {
        &mut self.radius
    }

    pub fn is_square_mut(&mut self) -> &mut bool {
        &mut self.is_square
    }

    pub fn get_matter(&self) -> u32 {
        self.matter
    }

    pub fn set_matter(&mut self, matter: u32) {
        self.matter = matter;
    }
}

impl EditorPainter {
    pub fn paint_round_line(
        &mut self,
        start: Option<Vec2>,
        end: Vec2,
        simulation: &mut Simulation,
    ) -> Result<()> {
        let start = start.unwrap_or(end);
        simulation.paint_round(start, end, self.matter, self.radius, self.is_square)
    }
}
