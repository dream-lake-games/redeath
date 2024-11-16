use crate::prelude::*;

mod animals;
mod items;
mod plants;
mod platforms;
mod world_loading;

pub use items::*;

pub(super) struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(platforms::PlatformsPlugin);

        animals::register_animals(app);
        items::register_items(app);
        plants::register_plants(app);
        world_loading::register_world_loading(app);
    }
}
