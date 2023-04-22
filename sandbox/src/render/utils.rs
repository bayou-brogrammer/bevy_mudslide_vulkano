use std::{collections::BTreeMap, sync::Arc};

use vulkano::{
    self,
    descriptor_set::layout::{
        DescriptorSetLayout, DescriptorSetLayoutBinding, DescriptorSetLayoutCreateInfo,
        DescriptorType,
    },
    device::Queue,
    pipeline::{layout::PipelineLayoutCreateInfo, ComputePipeline, PipelineLayout},
    shader::{EntryPoint, ShaderStages, SpecializationConstants},
};

/// Descriptor set layout binding information for storage buffer
pub fn storage_buffer_desc() -> DescriptorSetLayoutBinding {
    DescriptorSetLayoutBinding {
        stages: ShaderStages::COMPUTE,
        ..DescriptorSetLayoutBinding::descriptor_type(DescriptorType::StorageBuffer)
    }
}

/// Descriptor set layout binding information for image buffer
pub fn storage_image_desc() -> DescriptorSetLayoutBinding {
    DescriptorSetLayoutBinding {
        stages: ShaderStages::COMPUTE,
        ..DescriptorSetLayoutBinding::descriptor_type(DescriptorType::StorageImage)
    }
}

/// Creates a compute pipeline from given shader, with given descriptor layout binding.
/// The intention here is that the descriptor layout should correspond the shader's layout.
/// Normally you would use `ComputePipeline::new`, which would generate layout for descriptor
/// set automatically. However, because I've split the shaders to various different shaders,
/// each shader that does not use a variable from my shared layout don't get the bindings
/// for that specific variable. See https://github.com/vulkano-rs/vulkano/pull/1778 and https://github.com/vulkano-rs/vulkano/issues/1556#issuecomment-821658405.
pub fn create_compute_pipeline<Css>(
    compute_queue: Arc<Queue>,
    shader_entry_point: EntryPoint,
    descriptor_layout: Vec<(u32, DescriptorSetLayoutBinding)>,
    specialization_constants: &Css,
) -> Arc<ComputePipeline>
where
    Css: SpecializationConstants,
{
    let push_constant_reqs = shader_entry_point
        .push_constant_requirements()
        .cloned()
        .into_iter()
        .collect();
    let set_layout = DescriptorSetLayout::new(
        compute_queue.device().clone(),
        DescriptorSetLayoutCreateInfo {
            bindings: BTreeMap::from_iter(descriptor_layout),
            ..Default::default()
        },
    )
    .unwrap();
    let pipeline_layout =
        PipelineLayout::new(compute_queue.device().clone(), PipelineLayoutCreateInfo {
            set_layouts: vec![set_layout],
            push_constant_ranges: push_constant_reqs,
            ..Default::default()
        })
        .unwrap();
    ComputePipeline::with_pipeline_layout(
        compute_queue.device().clone(),
        shader_entry_point,
        specialization_constants,
        pipeline_layout,
        None,
    )
    .unwrap()
}
