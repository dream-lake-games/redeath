use crate::prelude::*;
use bevy::sprite::Material2dPlugin;

mod light_interaction;
pub mod light_manager;
pub mod light_mat;

pub use light_manager::*;
pub use light_mat::*;

#[derive(Resource, Clone, Debug, Reflect)]
pub struct BaseLights {
    pub ambience: Color,
    pub detail: Color,
    pub main: Color,
}

pub(super) struct LightPlugin;
impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<LightMat>::default());
        app.insert_resource(BaseLights {
            ambience: Color::srgba(0.4, 0.4, 0.4, 1.0),
            detail: Color::srgba(0.3, 0.3, 0.3, 1.0),
            main: Color::srgba(0.6, 0.6, 0.6, 1.0),
        });

        light_interaction::register_light_interaction(app);
        light_manager::register_light_manager(app);
    }
}
