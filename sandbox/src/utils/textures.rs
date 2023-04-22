use crate::matter::matter_definition::MatterDefinition;

pub fn gui_texture_rgba_data(matter: &MatterDefinition, dimensions: (u32, u32)) -> Vec<u8> {
    (0..(dimensions.0 * dimensions.1))
        .flat_map(|_| super::variated_color(matter.color.to_be_bytes()))
        .collect()
}
