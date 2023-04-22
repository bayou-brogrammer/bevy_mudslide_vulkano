use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Trait to control the projection matrix of a camera.
///
/// Components implementing this trait are automatically polled for changes, and used
/// to recompute the camera projection matrix of the [`Camera`] component attached to
/// the same entity as the component implementing this trait.
///
/// [`Camera`]: crate::camera::Camera
pub trait CameraProjection {
    fn get_projection_matrix(&self) -> Mat4;
    fn update(&mut self, width: f32, height: f32);
    fn far(&self) -> f32;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ScalingMode {
    /// Manually specify the projection's size, ignoring window resizing. The image will stretch.
    /// Arguments are in world units.
    Fixed { width: f32, height: f32 },
    /// Match the viewport size.
    /// The argument is the number of pixels that equals one world unit.
    WindowSize(f32),
    /// Keeping the aspect ratio while the axes can't be smaller than given minimum.
    /// Arguments are in world units.
    AutoMin { min_width: f32, min_height: f32 },
    /// Keeping the aspect ratio while the axes can't be bigger than given maximum.
    /// Arguments are in world units.
    AutoMax { max_width: f32, max_height: f32 },
    /// Keep the projection's height constant; width will be adjusted to match aspect ratio.
    /// The argument is the desired height of the projection in world units.
    FixedVertical(f32),
    /// Keep the projection's width constant; height will be adjusted to match aspect ratio.
    /// The argument is the desired width of the projection in world units.
    FixedHorizontal(f32),
}

/// Project a 3D space onto a 2D surface using parallel lines, i.e., unlike
/// [`PerspectiveProjection`], the size of objects remains the same regardless of their distance to
/// the camera.
///
/// The volume contained in the projection is called the *view frustum*. Since the viewport is
/// rectangular and projection lines are parallel, the view frustum takes the shape of a cuboid.
///
/// Note that the scale of the projection and the apparent size of objects are inversely
/// proportional. As the size of the projection increases, the size of objects decreases.
#[derive(Component, Debug, Clone, Copy)]
pub struct OrthographicProjection {
    /// The distance of the near clipping plane in world units.
    ///
    /// Objects closer than this will not be rendered.
    ///
    /// Defaults to `0.0`
    pub near: f32,

    /// The distance of the far clipping plane in world units.
    ///
    /// Objects further than this will not be rendered.
    ///
    /// Defaults to `1000.0`
    pub far: f32,

    /// How the projection will scale when the viewport is resized.
    ///
    /// Defaults to `ScalingMode::WindowScale(1.0)`
    // pub scaling_mode: ScalingMode,

    /// Scales the projection in world units.
    ///
    /// As scale increases, the apparent size of objects decreases, and vice versa.
    ///
    /// Defaults to `1.0`
    pub scale: f32,

    /// The area that the projection covers relative to `viewport_origin`.
    ///
    /// Bevy's [`camera_system`](crate::camera::camera_system) automatically
    /// updates this value when the viewport is resized depending on `OrthographicProjection`'s
    /// other fields. In this case, `area` should not be manually modified.
    ///
    /// It may be necessary to set this manually for shadow projections and such.
    // pub area: Rect,
    pub top: f32,
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
}

impl CameraProjection for OrthographicProjection {
    fn get_projection_matrix(&self) -> Mat4 {
        Mat4::orthographic_rh(
            self.left * self.scale,
            self.right * self.scale,
            self.bottom * self.scale,
            self.top * self.scale,
            self.near,
            self.far,
        )
    }

    fn update(&mut self, width: f32, height: f32) {
        let half_width = width / 2.0;
        let half_height = height / 2.0;
        self.left = -half_width;
        self.right = half_width;
        self.top = half_height;
        self.bottom = -half_height;
    }

    fn far(&self) -> f32 {
        self.far
    }
}

impl Default for OrthographicProjection {
    fn default() -> Self {
        OrthographicProjection {
            left: -1.0,
            right: 1.0,
            bottom: -1.0,
            top: 1.0,
            near: 0.0,
            far: 1000.0,
            scale: 1.0,
        }
    }
}
