use std::sync::Arc;

use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder,
        CommandBufferInheritanceInfo, CommandBufferUsage, SecondaryAutoCommandBuffer,
    },
    descriptor_set::{
        allocator::StandardDescriptorSetAllocator, PersistentDescriptorSet, WriteDescriptorSet,
    },
    device::{DeviceOwned, Queue},
    image::{ImageAccess, ImageViewAbstract},
    memory::allocator::{AllocationCreateInfo, MemoryUsage, StandardMemoryAllocator},
    pipeline::{
        graphics::{
            color_blend::ColorBlendState,
            input_assembly::InputAssemblyState,
            vertex_input::Vertex,
            viewport::{Viewport, ViewportState},
        },
        GraphicsPipeline, Pipeline, PipelineBindPoint,
    },
    render_pass::Subpass,
    sampler::{Filter, Sampler, SamplerAddressMode, SamplerCreateInfo, SamplerMipmapMode},
};

use super::camera::ortho_camera::OrthographicCamera;

/// Pipeline to draw pixel perfect images on quads
pub struct DrawQuadPipeline {
    quad: Mesh,
    subpass: Subpass,
    gfx_queue: Arc<Queue>,
    pipeline: Arc<GraphicsPipeline>,
    command_buffer_allocator: StandardCommandBufferAllocator,
    descriptor_set_allocator: StandardDescriptorSetAllocator,
}

impl DrawQuadPipeline {
    pub fn new(
        allocator: &Arc<StandardMemoryAllocator>,
        gfx_queue: Arc<Queue>,
        subpass: Subpass,
    ) -> DrawQuadPipeline {
        let quad = TexturedQuad::new(1.0, 1.0, [1.0; 4]).to_mesh(allocator);

        let pipeline = {
            let vs = vs::load(gfx_queue.device().clone()).expect("failed to create shader module");
            let fs = fs::load(gfx_queue.device().clone()).expect("failed to create shader module");
            GraphicsPipeline::start()
                .vertex_input_state(TexturedVertex::per_vertex())
                .vertex_shader(vs.entry_point("main").unwrap(), ())
                .input_assembly_state(InputAssemblyState::new())
                .fragment_shader(fs.entry_point("main").unwrap(), ())
                .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
                .render_pass(subpass.clone())
                .color_blend_state(ColorBlendState::default().blend_alpha())
                .build(gfx_queue.device().clone())
                .unwrap()
        };

        DrawQuadPipeline {
            gfx_queue,
            pipeline,
            subpass,
            quad,
            command_buffer_allocator: StandardCommandBufferAllocator::new(
                allocator.device().clone(),
                Default::default(),
            ),
            descriptor_set_allocator: StandardDescriptorSetAllocator::new(
                allocator.device().clone(),
            ),
        }
    }

    pub fn create_image_sampler_nearest_descriptor_set(
        &self,
        image: Arc<dyn ImageViewAbstract>,
    ) -> Arc<PersistentDescriptorSet> {
        let layout = self.pipeline.layout().set_layouts().get(0).unwrap();
        let sampler = Sampler::new(
            self.gfx_queue.device().clone(),
            SamplerCreateInfo {
                mag_filter: Filter::Nearest,
                min_filter: Filter::Nearest,
                address_mode: [SamplerAddressMode::Repeat; 3],
                mipmap_mode: SamplerMipmapMode::Nearest,
                ..Default::default()
            },
        )
        .unwrap();
        PersistentDescriptorSet::new(
            &self.descriptor_set_allocator,
            layout.clone(),
            [WriteDescriptorSet::image_view_sampler(
                0,
                image.clone(),
                sampler,
            )],
        )
        .unwrap()
    }

    /// Draw input `image` on a quad at (0.0, 0.0), between -1.0 and 1.0
    pub fn draw(
        &mut self,
        viewport_dimensions: [u32; 2],
        camera: OrthographicCamera,
        image: Arc<dyn ImageViewAbstract>,
        flip_x: bool,
        flip_y: bool,
    ) -> SecondaryAutoCommandBuffer {
        // Command buffer for our single subpass
        let mut builder = AutoCommandBufferBuilder::secondary(
            &self.command_buffer_allocator,
            self.gfx_queue.queue_family_index(),
            CommandBufferUsage::MultipleSubmit,
            CommandBufferInheritanceInfo {
                render_pass: Some(self.subpass.clone().into()),
                ..Default::default()
            },
        )
        .unwrap();

        let dims = image.image().dimensions();
        let push_constants = vs::PushConstants {
            world_to_screen: camera.world_to_screen().to_cols_array_2d(),
            // Scale transforms our 1.0 sized quad to actual pixel size in screen space
            scale: [
                dims.width() as f32 * if flip_x { -1.0 } else { 1.0 },
                dims.height() as f32 * if flip_y { -1.0 } else { 1.0 },
            ],
        };

        let image_sampler_descriptor_set = self.create_image_sampler_nearest_descriptor_set(image);
        builder
            .set_viewport(
                0,
                [Viewport {
                    origin: [0.0, 0.0],
                    dimensions: [viewport_dimensions[0] as f32, viewport_dimensions[1] as f32],
                    depth_range: 0.0..1.0,
                }],
            )
            .bind_pipeline_graphics(self.pipeline.clone())
            .bind_descriptor_sets(
                PipelineBindPoint::Graphics,
                self.pipeline.layout().clone(),
                0,
                image_sampler_descriptor_set,
            )
            .push_constants(self.pipeline.layout().clone(), 0, push_constants)
            .bind_vertex_buffers(0, self.quad.vertices.clone())
            .bind_index_buffer(self.quad.indices.clone())
            .draw_indexed(self.quad.indices.len() as u32, 1, 0, 0, 0)
            .unwrap();
        builder.build().unwrap()
    }
}

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "shaders/quad_vert.glsl",
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "shaders/quad_frag.glsl",
    }
}

/// Vertex for textured quads.
#[repr(C)]
#[derive(Default, Debug, Copy, Clone, BufferContents, Vertex)]
pub struct TexturedVertex {
    #[format(R32G32B32A32_SFLOAT)]
    pub color: [f32; 4],
    #[format(R32G32_SFLOAT)]
    pub position: [f32; 2],
    #[format(R32G32_SFLOAT)]
    pub tex_coords: [f32; 2],
}

/// Textured quad with vertices & indices
#[derive(Default, Debug, Copy, Clone)]
pub struct TexturedQuad {
    pub vertices: [TexturedVertex; 4],
    pub indices: [u32; 6],
}

/// A set of vertices and their indices as cpu accessible buffers
#[derive(Clone)]
pub struct Mesh {
    pub indices: Subbuffer<[u32]>,
    pub vertices: Subbuffer<[TexturedVertex]>,
}

impl TexturedQuad {
    /// Creates a new textured quad with given width and height at (0.0, 0.0)
    pub fn new(width: f32, height: f32, color: [f32; 4]) -> TexturedQuad {
        TexturedQuad {
            vertices: [
                TexturedVertex {
                    position: [-(width / 2.0), -(height / 2.0)],
                    tex_coords: [0.0, 1.0],
                    color,
                },
                TexturedVertex {
                    position: [-(width / 2.0), height / 2.0],
                    tex_coords: [0.0, 0.0],
                    color,
                },
                TexturedVertex {
                    position: [width / 2.0, height / 2.0],
                    tex_coords: [1.0, 0.0],
                    color,
                },
                TexturedVertex {
                    position: [width / 2.0, -(height / 2.0)],
                    tex_coords: [1.0, 1.0],
                    color,
                },
            ],
            indices: [0, 2, 1, 0, 3, 2],
        }
    }

    /// Converts Quad data to a mesh that can be used in drawing
    pub fn to_mesh(self, allocator: &Arc<StandardMemoryAllocator>) -> Mesh {
        Mesh {
            vertices: Buffer::from_iter(
                allocator,
                BufferCreateInfo {
                    usage: BufferUsage::VERTEX_BUFFER,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    usage: MemoryUsage::Upload,
                    ..Default::default()
                },
                self.vertices.into_iter(),
            )
            .unwrap(),
            indices: Buffer::from_iter(
                allocator,
                BufferCreateInfo {
                    usage: BufferUsage::INDEX_BUFFER,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    usage: MemoryUsage::Upload,
                    ..Default::default()
                },
                self.indices.into_iter(),
            )
            .unwrap(),
        }
    }
}
