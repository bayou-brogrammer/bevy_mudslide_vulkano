use std::io::Cursor;

use anyhow::{Context, Result};
use bevy::{
    app::PluginGroupBuilder,
    log::Level,
    prelude::*,
    window::{PrimaryWindow, WindowMode},
};
use bevy_fn_plugin::bevy_plugin;
use bevy_mod_sysfail::macros::*;
use bevy_vulkano::{BevyVulkanoSettings, BevyVulkanoWindows, VulkanoWinitPlugin};

pub const WIDTH: f32 = 1920.0;
pub const HEIGHT: f32 = 1080.0;

pub struct VulkanBundle;
impl PluginGroup for VulkanBundle {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            // Minimum plugins for the demo
            .add(bevy::log::LogPlugin {
                level: Level::INFO,
                filter: "wgpu=error".to_string(),
            })
            .add(bevy::time::TimePlugin::default())
            .add(bevy::input::InputPlugin::default())
            .add(bevy::window::WindowPlugin::default())
            .add(bevy::asset::AssetPlugin::default())
            .add(bevy::core::TaskPoolPlugin::default())
            .add(bevy::core::FrameCountPlugin::default())
            .add(bevy::core::TypeRegistrationPlugin::default())
            .add(bevy::diagnostic::DiagnosticsPlugin::default())
            .add(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
            // Don't add WinitPlugin. This owns "core loop" (runner).
            // Bevy winit and render should be excluded
            .add(VulkanoWinitPlugin::default())
    }
}

/// Overrides the default Bevy plugins and configures things like the screen settings.
#[bevy_plugin]
pub fn BevyConfigPlugin(app: &mut App) {
    let default_plugins = VulkanBundle.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (WIDTH, HEIGHT).into(),
            title: "Bevy Vulkano Game Of Life".to_string(),
            present_mode: bevy::window::PresentMode::Immediate,
            resizable: true,
            mode: WindowMode::Windowed,
            ..default()
        }),
        ..default()
    });

    #[cfg(feature = "native-dev")]
    let default_plugins = default_plugins.set(AssetPlugin {
        watch_for_changes: true,
        asset_folder: "../assets".to_string(),
    });

    app.insert_non_send_resource(BevyVulkanoSettings {
        // Since we're only drawing gui, let's clear each frame
        is_gui_overlay: true,
        ..BevyVulkanoSettings::default()
    })
    .add_plugins(default_plugins)
    .add_system(set_window_icon.on_startup());
}

// Sets the icon on Windows and X11
#[sysfail(log(level = "error"))]
fn set_window_icon(
    vulkan_windows: NonSend<BevyVulkanoWindows>,
    primary_windows: Query<Entity, With<PrimaryWindow>>,
) -> Result<()> {
    use winit::window::Icon;

    let primary_entity = primary_windows.single();
    let primary = vulkan_windows
        .get_vulkano_window(primary_entity)
        .context("Failed to get primary vulkan window")?;

    let icon_buf = Cursor::new(include_bytes!(
        "../../build/macos/AppIcon.iconset/icon_256x256.png"
    ));

    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        let icon = Icon::from_rgba(rgba, width, height)?;
        primary.window().set_window_icon(Some(icon));
    };

    Ok(())
}
