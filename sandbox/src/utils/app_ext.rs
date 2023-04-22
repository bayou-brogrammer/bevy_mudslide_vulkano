use bevy::prelude::*;

pub trait AppExt {
    fn init_resource_on_enter<S: States, A: Resource + FromWorld>(
        &mut self,
        loading_state: S,
    ) -> &mut Self;

    fn init_resource_on_exit<S: States, A: Resource + FromWorld>(
        &mut self,
        loading_state: S,
    ) -> &mut Self;
}

impl AppExt for App {
    fn init_resource_on_enter<S: States, A: Resource + FromWorld>(
        &mut self,
        loading_state: S,
    ) -> &mut Self {
        self.add_system(init_resource::<A>.in_schedule(OnEnter(loading_state)))
    }

    fn init_resource_on_exit<S: States, A: Resource + FromWorld>(
        &mut self,
        loading_state: S,
    ) -> &mut Self {
        self.add_system(init_resource::<A>.in_schedule(OnExit(loading_state)))
    }
}

pub(crate) fn init_resource<Asset: Resource + FromWorld>(world: &mut World) {
    let asset = Asset::from_world(world);
    world.insert_resource(asset);
}
