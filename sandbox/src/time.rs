pub mod performance_timer;

use bevy::prelude::Resource;
use bevy_fn_plugin::bevy_plugin;

use self::performance_timer::PerformanceTimer;

const NUM_TIME_SAMPLES: usize = 150;

#[bevy_plugin]
pub fn TimerPlugin(app: &mut App) {
    app.init_resource::<SimulationTimer>()
        .init_resource::<RenderTimer>();
}

#[derive(Resource, Default)]
pub struct SimulationTimer(pub PerformanceTimer);

#[derive(Resource, Default)]
pub struct RenderTimer(pub PerformanceTimer);
