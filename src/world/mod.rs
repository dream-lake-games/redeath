use crate::prelude::*;

mod animals;
pub mod cleanup;
mod items;
mod plants;
mod platforms;
mod world_loading;

pub use cleanup::*;
pub use items::*;

pub(super) struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(platforms::PlatformsPlugin);

        animals::register_animals(app);
        cleanup::register_cleanup(app);
        items::register_items(app);
        plants::register_plants(app);
        world_loading::register_world_loading(app);
    }
}
