pub mod ortho_camera;
pub mod projection;
pub use ortho_camera::OrthographicCamera;

use bevy::{input::mouse::MouseWheel, prelude::*, window::WindowResized};
use bevy_fn_plugin::bevy_plugin;

use crate::{bevy_config::HEIGHT, GameState, CAMERA_MOVE_SPEED, SIM_CANVAS_SIZE};

#[bevy_plugin]
pub fn CameraPlugin(app: &mut App) {
    app.add_startup_system(setup_camera).add_systems(
        (update_camera, camera_controls).distributive_run_if(in_state(GameState::Simulating)),
    );
}

/// Creates our simulation & render pipelines
fn setup_camera(window_query: Query<&Window>, mut commands: Commands) {
    let window = window_query.single();

    // Create simple orthographic camera
    let mut camera = OrthographicCamera::default();

    // Update camera to window size
    camera.update(window.width(), window.height());

    // Zoom camera to fit vertical pixels
    camera.zoom_to_fit_pixels(SIM_CANVAS_SIZE, HEIGHT as u32);

    // Insert resources
    commands.insert_resource(camera);
}

/// Update camera (if window is resized)
fn update_camera(
    mut camera: ResMut<OrthographicCamera>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for ev in resize_reader.iter() {
        camera.update(ev.width, ev.height);
    }
}

/// Input actions for camera movement, zoom and pausing
fn camera_controls(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut camera: ResMut<OrthographicCamera>,
    mut mouse_input_events: EventReader<MouseWheel>,
) {
    // Move camera with arrows & WASD
    let up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
    let down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
    let left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
    let right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);

    let x_axis = -(right as i8) + left as i8;
    let y_axis = -(up as i8) + down as i8;

    let mut move_delta = Vec2::new(x_axis as f32, y_axis as f32);
    if move_delta != Vec2::ZERO {
        move_delta /= move_delta.length();
        camera.translate(move_delta * time.delta_seconds() * CAMERA_MOVE_SPEED);
    }

    // Zoom camera with mouse scroll
    for e in mouse_input_events.iter() {
        if e.y < 0.0 {
            camera.zoom(1.05)
        } else {
            camera.zoom(1.0 / 1.05);
        }
    }
}
