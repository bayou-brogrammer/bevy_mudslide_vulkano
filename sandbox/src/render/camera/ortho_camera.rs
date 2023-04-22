use bevy::prelude::*;

use super::projection::{CameraProjection, OrthographicProjection};

const PIXEL_MULTIPLIER: f32 = 1.1;

// c1r1: y flipped for vulkan
#[rustfmt::skip]
pub const OPENGL_TO_VULKAN_MATRIX: Mat4 = Mat4::from_cols_array(&[
    1.0, 0.0, 0.0, 0.0,
    0.0, -1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
]);

const Z_POS: f32 = -10.0;

/// A simple orthographic camera
#[derive(Debug, Copy, Clone, Resource)]
pub struct OrthographicCamera {
    pub pos: Vec2,
    pub ortho: OrthographicProjection,
}

impl OrthographicCamera {
    pub fn zoom(&mut self, zoom: f32) {
        self.ortho.scale *= zoom;
    }

    pub fn scale(&self) -> f32 {
        self.ortho.scale
    }

    /// Translates camera position
    pub fn translate(&mut self, translation: Vec2) {
        self.pos += translation;
    }

    /// After window size changes, update our camera
    pub fn update(&mut self, width: f32, height: f32) {
        self.ortho.update(width, height);
    }

    /// Get world to screen matrix to be passed to our rendering
    pub fn world_to_screen(&self) -> Mat4 {
        OPENGL_TO_VULKAN_MATRIX
            * self.ortho.get_projection_matrix()
            * Transform::from_translation(self.pos.extend(Z_POS)).compute_matrix()
    }

    /// A matrix4 that transforms screen coordinates fo world coordinates
    #[allow(dead_code)]
    pub fn screen_to_world(&self) -> Mat4 {
        self.world_to_screen().inverse()
    }

    pub fn screen_to_world_pos(&self, window: &Window, normalized_window_pos: Vec2) -> Vec2 {
        let updated_pos = Vec2::new(
            normalized_window_pos.x - window.width() / 2.0,
            normalized_window_pos.y - window.height() / 2.0,
        ) * Vec2::new(1.0, -1.0);

        updated_pos * self.scale() - self.pos
    }

    pub fn reset_zoom(&mut self) {
        self.zoom(1.0 / self.ortho.scale);
    }

    /// Approximately zoom to fit our canvas size so that it's large enough in the beginning
    pub fn zoom_to_fit_pixels(&mut self, visible_pixels: u32, actual_pixels: u32) {
        self.reset_zoom();
        self.zoom(visible_pixels as f32 / (actual_pixels as f32 / PIXEL_MULTIPLIER))
    }
}

impl Default for OrthographicCamera {
    fn default() -> Self {
        OrthographicCamera {
            pos: Vec2::new(0.0, 0.0),
            ortho: OrthographicProjection::default(),
        }
    }
}
