use std::sync::Arc;

use anyhow::Result;
use bevy::{math::IVec2, prelude::Vec2, utils::Instant};
use vulkano::{
    buffer::Subbuffer,
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
        PrimaryAutoCommandBuffer, PrimaryCommandBufferAbstract,
    },
    descriptor_set::{
        allocator::StandardDescriptorSetAllocator, PersistentDescriptorSet, WriteDescriptorSet,
    },
    device::{DeviceOwned, Queue},
    format::Format,
    image::{ImageUsage, StorageImage},
    memory::allocator::StandardMemoryAllocator,
    pipeline::{ComputePipeline, Pipeline, PipelineBindPoint},
    sync::GpuFuture,
};
use vulkano_util::renderer::DeviceImageView;

use crate::{
    matter::{
        matter_definition::MatterDefinitions, matter_state::MatterState, MatterWithColor,
        MAX_NUM_MATTERS,
    },
    render::utils::{create_compute_pipeline, storage_buffer_desc, storage_image_desc},
    settings::AppSettings,
    simulator::gpu_utils::{empty_f32, empty_u32, empty_with},
    utils::is_inside_sim_canvas,
    KERNEL_SIZE, NUM_WORK_GROUPS, SIM_CANVAS_SIZE,
};

struct Pipelines {
    color_pipeline: Arc<ComputePipeline>,
    rise_swap_pipeline: Arc<ComputePipeline>,
    fall_swap_pipeline: Arc<ComputePipeline>,
    fall_empty_pipeline: Arc<ComputePipeline>,
    rise_empty_pipeline: Arc<ComputePipeline>,
    draw_matter_pipeline: Arc<ComputePipeline>,
    query_matter_pipeline: Arc<ComputePipeline>,
    slide_down_swap_pipeline: Arc<ComputePipeline>,
    horizontal_swap_pipeline: Arc<ComputePipeline>,
    horizontal_empty_pipeline: Arc<ComputePipeline>,
    slide_down_empty_pipeline: Arc<ComputePipeline>,
}

/// Cellular automata simulation pipeline
pub struct CASimulator {
    // Push constants
    seed: f32,
    sim_steps: u32,
    move_step: u32,
    draw_radius: f32,
    query_pos: IVec2,
    empty_matter: u32,
    draw_pos_end: Vec2,
    dispersion_dir: u32,
    dispersion_step: u32,
    draw_pos_start: Vec2,
    draw_matter: MatterWithColor,

    // Shader matter inputs
    image: DeviceImageView,
    matter_out: Subbuffer<[u32]>,
    query_matter: Subbuffer<[u32]>,
    pub matter_in: Subbuffer<[u32]>,
    matter_state_input: Subbuffer<[u32]>,
    matter_weight_input: Subbuffer<[f32]>,
    matter_dispersion_input: Subbuffer<[u32]>,

    // Pipelines
    color_pipeline: Arc<ComputePipeline>,
    draw_matter_pipeline: Arc<ComputePipeline>,
    query_matter_pipeline: Arc<ComputePipeline>,

    rise_swap_pipeline: Arc<ComputePipeline>,
    fall_swap_pipeline: Arc<ComputePipeline>,
    fall_empty_pipeline: Arc<ComputePipeline>,
    rise_empty_pipeline: Arc<ComputePipeline>,
    slide_down_swap_pipeline: Arc<ComputePipeline>,
    horizontal_swap_pipeline: Arc<ComputePipeline>,
    horizontal_empty_pipeline: Arc<ComputePipeline>,
    slide_down_empty_pipeline: Arc<ComputePipeline>,

    // Misc
    start: Instant,
    compute_queue: Arc<Queue>,
    matter_definitions: MatterDefinitions,
    command_buffer_allocator: StandardCommandBufferAllocator,
    descriptor_set_allocator: StandardDescriptorSetAllocator,
}

impl CASimulator {
    /// Create new simulator pipeline for a compute queue. Ensure that canvas sizes are divisible by
    /// kernel sizes so no pixel remains unsimulated.
    pub fn new(
        allocator: &Arc<StandardMemoryAllocator>,
        compute_queue: Arc<Queue>,
        matter_definitions: &MatterDefinitions,
    ) -> Result<CASimulator> {
        // In order to not miss any pixels, the following must be true
        assert_eq!(SIM_CANVAS_SIZE % KERNEL_SIZE, 0);

        let matter_in = empty_u32(allocator, (SIM_CANVAS_SIZE * SIM_CANVAS_SIZE) as usize)?;
        let matter_out = empty_u32(allocator, (SIM_CANVAS_SIZE * SIM_CANVAS_SIZE) as usize)?;
        let query_matter = empty_with(allocator, vec![MatterWithColor::from(0).value])?;
        let matter_state_input = empty_u32(allocator, MAX_NUM_MATTERS as usize)?;
        let matter_weight_input = empty_f32(allocator, MAX_NUM_MATTERS as usize)?;
        let matter_dispersion_input = empty_u32(allocator, MAX_NUM_MATTERS as usize)?;

        // Create color image
        let image = StorageImage::general_purpose_image_view(
            allocator,
            compute_queue.clone(),
            [SIM_CANVAS_SIZE, SIM_CANVAS_SIZE],
            Format::R8G8B8A8_UNORM,
            ImageUsage::SAMPLED | ImageUsage::STORAGE | ImageUsage::TRANSFER_DST,
        )?;

        // Create pipelines
        let Pipelines {
            color_pipeline,
            rise_swap_pipeline,
            fall_swap_pipeline,
            rise_empty_pipeline,
            fall_empty_pipeline,
            draw_matter_pipeline,
            query_matter_pipeline,
            horizontal_swap_pipeline,
            slide_down_swap_pipeline,
            horizontal_empty_pipeline,
            slide_down_empty_pipeline,
        } = CASimulator::create_pipelines(&compute_queue, matter_definitions.empty)?;

        Ok(CASimulator {
            // Push constants
            image,
            seed: 0.0,
            sim_steps: 0,
            move_step: 0,
            draw_radius: 0.0,
            dispersion_dir: 0,
            dispersion_step: 0,
            query_pos: IVec2::new(0, 0),
            draw_pos_end: Vec2::new(0.0, 0.0),
            draw_pos_start: Vec2::new(0.0, 0.0),
            draw_matter: MatterWithColor::from(0),
            empty_matter: matter_definitions.empty,

            // Shader matter inputs
            matter_in,
            matter_out,
            query_matter,
            matter_state_input,
            matter_weight_input,
            matter_dispersion_input,

            // Pipelines
            color_pipeline,
            fall_swap_pipeline,
            rise_swap_pipeline,
            rise_empty_pipeline,
            fall_empty_pipeline,
            draw_matter_pipeline,
            query_matter_pipeline,
            horizontal_swap_pipeline,
            slide_down_swap_pipeline,
            horizontal_empty_pipeline,
            slide_down_empty_pipeline,

            // Misc
            compute_queue,
            start: Instant::now(),
            matter_definitions: matter_definitions.clone(),
            command_buffer_allocator: StandardCommandBufferAllocator::new(
                allocator.device().clone(),
                Default::default(),
            ),
            descriptor_set_allocator: StandardDescriptorSetAllocator::new(
                allocator.device().clone(),
            ),
        })
    }
}

impl CASimulator {
    fn create_pipelines(compute_queue: &Arc<Queue>, empty_matter: u32) -> Result<Pipelines> {
        let spec_const = color_cs::SpecializationConstants {
            empty_matter,
            constant_8: KERNEL_SIZE,
            constant_9: KERNEL_SIZE,
            state_gas: MatterState::Gas as u32,
            state_solid: MatterState::Solid as u32,
            state_empty: MatterState::Empty as u32,
            state_powder: MatterState::Powder as u32,
            state_liquid: MatterState::Liquid as u32,
            state_solid_gravity: MatterState::SolidGravity as u32,
            sim_canvas_size: SIM_CANVAS_SIZE as i32,
        };

        // This must match the shader & inputs in dispatch
        let descriptor_layout = [
            (0, storage_buffer_desc()),
            (1, storage_buffer_desc()),
            (2, storage_buffer_desc()),
            (3, storage_buffer_desc()),
            (4, storage_buffer_desc()),
            (5, storage_buffer_desc()),
            (6, storage_image_desc()),
        ];

        let fall_empty_pipeline = {
            let fall_shader = fall_empty_cs::load(compute_queue.device().clone())?;
            create_compute_pipeline(
                compute_queue.clone(),
                fall_shader.entry_point("main").unwrap(),
                descriptor_layout.to_vec(),
                &spec_const,
            )
        };
        let fall_swap_pipeline = {
            let fall_swap_shader = fall_swap_cs::load(compute_queue.device().clone())?;
            create_compute_pipeline(
                compute_queue.clone(),
                fall_swap_shader.entry_point("main").unwrap(),
                descriptor_layout.to_vec(),
                &spec_const,
            )
        };
        let horizontal_empty_pipeline = {
            let shader = horizontal_empty_cs::load(compute_queue.device().clone())?;
            create_compute_pipeline(
                compute_queue.clone(),
                shader.entry_point("main").unwrap(),
                descriptor_layout.to_vec(),
                &spec_const,
            )
        };
        let horizontal_swap_pipeline = {
            let shader = horizontal_swap_cs::load(compute_queue.device().clone())?;
            create_compute_pipeline(
                compute_queue.clone(),
                shader.entry_point("main").unwrap(),
                descriptor_layout.to_vec(),
                &spec_const,
            )
        };
        let slide_down_empty_pipeline = {
            let shader = slide_down_empty_cs::load(compute_queue.device().clone())?;
            create_compute_pipeline(
                compute_queue.clone(),
                shader.entry_point("main").unwrap(),
                descriptor_layout.to_vec(),
                &spec_const,
            )
        };
        let slide_down_swap_pipeline = {
            let shader = slide_down_swap_cs::load(compute_queue.device().clone())?;
            create_compute_pipeline(
                compute_queue.clone(),
                shader.entry_point("main").unwrap(),
                descriptor_layout.to_vec(),
                &spec_const,
            )
        };
        let rise_empty_pipeline = {
            let shader = rise_empty_cs::load(compute_queue.device().clone())?;
            create_compute_pipeline(
                compute_queue.clone(),
                shader.entry_point("main").unwrap(),
                descriptor_layout.to_vec(),
                &spec_const,
            )
        };
        let rise_swap_pipeline = {
            let shader = rise_swap_cs::load(compute_queue.device().clone())?;
            create_compute_pipeline(
                compute_queue.clone(),
                shader.entry_point("main").unwrap(),
                descriptor_layout.to_vec(),
                &spec_const,
            )
        };

        let draw_matter_pipeline = {
            let draw_matter_shader = draw_matter_cs::load(compute_queue.device().clone())?;
            create_compute_pipeline(
                compute_queue.clone(),
                draw_matter_shader.entry_point("main").unwrap(),
                descriptor_layout.to_vec(),
                &spec_const,
            )
        };

        let query_matter_pipeline = {
            let query_matter_shader = query_matter_cs::load(compute_queue.device().clone())?;
            create_compute_pipeline(
                compute_queue.clone(),
                query_matter_shader.entry_point("main").unwrap(),
                descriptor_layout.to_vec(),
                &spec_const,
            )
        };

        let color_pipeline = {
            let color_shader = color_cs::load(compute_queue.device().clone())?;
            create_compute_pipeline(
                compute_queue.clone(),
                color_shader.entry_point("main").unwrap(),
                descriptor_layout.to_vec(),
                &spec_const,
            )
        };

        Ok(Pipelines {
            color_pipeline,
            rise_swap_pipeline,
            fall_swap_pipeline,
            rise_empty_pipeline,
            fall_empty_pipeline,
            draw_matter_pipeline,
            query_matter_pipeline,
            horizontal_swap_pipeline,
            horizontal_empty_pipeline,
            slide_down_swap_pipeline,
            slide_down_empty_pipeline,
        })
    }
}

// Query
impl CASimulator {
    /// Get canvas image for rendering
    pub fn color_image(&self) -> DeviceImageView {
        self.image.clone()
    }

    /// Draw matter line with given radius
    pub fn draw_matter(
        &mut self,
        start: Vec2,
        end: Vec2,
        matter: u32,
        radius: f32,
        is_square: bool,
    ) {
        // Update our variables to be used as push constants
        self.draw_pos_start = start;
        self.draw_pos_end = end;
        self.draw_radius = radius;
        self.draw_matter = MatterWithColor::new(
            matter,
            crate::utils::u32_rgba_to_u8_rgba(
                self.matter_definitions.definitions[matter as usize].color,
            ),
        );

        // Build command buffer
        let mut command_buffer_builder = self.command_buffer_builder();

        // Dispatch
        self.dispatch(
            &mut command_buffer_builder,
            self.draw_matter_pipeline.clone(),
            is_square,
            false,
        );

        // Execute & finish (no need to wait)
        self.execute(command_buffer_builder, false);
    }

    /// Query matter at pos
    pub fn query_matter(&mut self, pos: IVec2) -> Option<u32> {
        if is_inside_sim_canvas(pos) {
            self.query_pos = pos;
            // Build command buffer
            let mut command_buffer_builder = self.command_buffer_builder();

            // Dispatch
            self.dispatch(
                &mut command_buffer_builder,
                self.query_matter_pipeline.clone(),
                false,
                false,
            );

            // Execute & finish (wait)
            self.execute(command_buffer_builder, true);

            // Read result
            let query_matter = self.query_matter.read().unwrap();
            Some(MatterWithColor::from(query_matter[0]).matter_id())
        } else {
            None
        }
    }

    pub(crate) fn update_matter_data(
        &mut self,
        matter_definitions: &MatterDefinitions,
    ) -> Result<()> {
        let mut write_matter_state_input = self.matter_state_input.write()?;
        let mut write_matter_weight_input = self.matter_weight_input.write()?;
        let mut write_matter_dispersion_input = self.matter_dispersion_input.write()?;

        matter_definitions.definitions.iter().for_each(|def| {
            write_matter_state_input[def.id as usize] = def.state as u32;
            write_matter_weight_input[def.id as usize] = def.weight;
            write_matter_dispersion_input[def.id as usize] = def.dispersion;
        });

        self.empty_matter = matter_definitions.empty;
        self.matter_definitions = matter_definitions.clone();

        Ok(())
    }
}

// Simulation
impl CASimulator {
    fn command_buffer_builder(&self) -> AutoCommandBufferBuilder<PrimaryAutoCommandBuffer> {
        AutoCommandBufferBuilder::primary(
            &self.command_buffer_allocator,
            self.compute_queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap()
    }

    fn execute(
        &self,
        command_buffer_builder: AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
        wait: bool,
    ) {
        let command_buffer = command_buffer_builder.build().unwrap();
        let finished = command_buffer.execute(self.compute_queue.clone()).unwrap();
        let future = finished.then_signal_fence_and_flush().unwrap();
        if wait {
            future.wait(None).unwrap();
        }
    }

    /// Step simulation
    pub fn step(&mut self, settings: &AppSettings) {
        self.seed = (Instant::now() - self.start).as_secs_f32();

        let mut builder = self.command_buffer_builder();

        if !settings.is_paused {
            // Movement
            // ------
            self.move_once(&mut builder, 0);
            self.disperse(
                &mut builder,
                (self.sim_steps % 2 == 0) as u32,
                settings.dispersion_steps,
            );
            if settings.movement_steps > 1 {
                self.move_once(&mut builder, 1);
            }
            if settings.movement_steps > 2 {
                self.move_once(&mut builder, 2);
            }
            self.disperse(
                &mut builder,
                (self.sim_steps % 2 != 0) as u32,
                settings.dispersion_steps,
            );
            // ------

            // React
            // self.dispatch(&mut builder, self.react_pipeline.clone(), true)?;
        }

        // Finally color the image
        self.dispatch(&mut builder, self.color_pipeline.clone(), false, false);

        // Execute & finish (no need to wait)
        self.execute(builder, false);
        self.sim_steps += 1;
    }

    /// Step a movement pipeline. move_step affects the order of sliding direction
    fn move_once(
        &mut self,
        builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
        step: u32,
    ) {
        self.move_step = step;

        // Anything that falls
        self.dispatch(builder, self.fall_empty_pipeline.clone(), false, true);
        self.dispatch(builder, self.fall_swap_pipeline.clone(), false, true);

        // Risers
        self.dispatch(builder, self.rise_empty_pipeline.clone(), false, true);
        self.dispatch(builder, self.rise_swap_pipeline.clone(), false, true);

        // Sliders
        self.dispatch(builder, self.slide_down_empty_pipeline.clone(), false, true);
        self.dispatch(builder, self.slide_down_swap_pipeline.clone(), false, true);
    }

    fn disperse(
        &mut self,
        builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
        direction: u32,
        dispersion_steps: u32,
    ) {
        self.dispersion_dir = direction;
        for dispersion_step in 0..dispersion_steps {
            self.dispersion_step = dispersion_step;
            self.dispatch(builder, self.horizontal_empty_pipeline.clone(), false, true);
            self.dispatch(builder, self.horizontal_swap_pipeline.clone(), false, true);
        }
    }

    /// Append a pipeline dispatch to our command buffer
    fn dispatch(
        &mut self,
        builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
        pipeline: Arc<ComputePipeline>,
        is_square: bool,
        swap: bool,
    ) {
        let pipeline_layout = pipeline.layout();
        let desc_layout = pipeline_layout.set_layouts().get(0).unwrap();
        let set = PersistentDescriptorSet::new(
            &self.descriptor_set_allocator,
            desc_layout.clone(),
            [
                WriteDescriptorSet::buffer(0, self.matter_state_input.clone()),
                WriteDescriptorSet::buffer(1, self.matter_weight_input.clone()),
                WriteDescriptorSet::buffer(2, self.matter_dispersion_input.clone()),
                WriteDescriptorSet::buffer(3, self.matter_in.clone()),
                WriteDescriptorSet::buffer(4, self.matter_out.clone()),
                WriteDescriptorSet::buffer(5, self.query_matter.clone()),
                WriteDescriptorSet::image_view(6, self.image.clone()),
            ],
        )
        .unwrap();

        let push_constants = fall_empty_cs::PushConstants {
            is_square: is_square as u32,
            seed: self.seed,
            sim_steps: self.sim_steps,
            move_step: self.move_step,
            draw_radius: self.draw_radius,
            query_pos: self.query_pos.into(),
            draw_matter: self.draw_matter.value,
            dispersion_dir: self.dispersion_dir,
            dispersion_step: self.dispersion_step,
            draw_pos_end: self.draw_pos_end.into(),
            draw_pos_start: self.draw_pos_start.into(),
        };

        builder
            .bind_pipeline_compute(pipeline.clone())
            .bind_descriptor_sets(PipelineBindPoint::Compute, pipeline_layout.clone(), 0, set)
            .push_constants(pipeline_layout.clone(), 0, push_constants)
            .dispatch([NUM_WORK_GROUPS, NUM_WORK_GROUPS, 1])
            .unwrap();

        // Double buffering: Swap input and output so the output becomes the input for next frame
        if swap {
            std::mem::swap(&mut self.matter_in, &mut self.matter_out);
        }
    }
}

// Fall Shaders
mod fall_empty_cs {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "compute_shaders/empty/fall_empty.glsl"
    }
}
mod fall_swap_cs {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "compute_shaders/swap/fall_swap.glsl",
    }
}

// Horizontal Shaders
mod horizontal_empty_cs {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "compute_shaders/empty/horizontal_empty.glsl",
    }
}

#[allow(deprecated)]
mod horizontal_swap_cs {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "compute_shaders/swap/horizontal_swap.glsl",
    }
}

// Slide
mod slide_down_empty_cs {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "compute_shaders/empty/slide_down_empty.glsl",
    }
}
mod slide_down_swap_cs {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "compute_shaders/swap/slide_down_swap.glsl",
    }
}

// Rise
mod rise_empty_cs {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "compute_shaders/empty/rise_empty.glsl",
    }
}

mod rise_swap_cs {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "compute_shaders/swap/rise_swap.glsl",
    }
}

// React
// Render
mod color_cs {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "compute_shaders/color.glsl"
    }
}

mod draw_matter_cs {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "compute_shaders/draw_matter.glsl"
    }
}

mod query_matter_cs {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "compute_shaders/query_matter.glsl"
    }
}
