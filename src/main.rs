pub mod consts;
pub mod debug;
pub mod layer;
pub mod lazy;
pub mod palette;
pub mod root;
pub mod state;

pub mod prelude {
    pub use super::{consts::*, layer::*, lazy::*, palette::*, root::*, state::*};
    pub use bevy::{
        color::palettes::tailwind,
        ecs::component::StorageType,
        input::common_conditions::input_toggle_active,
        math::VectorSpace,
        prelude::*,
        reflect::GetTypeRegistration,
        render::view::RenderLayers,
        utils::{HashMap, HashSet},
    };
    pub use bevy_ecs_ldtk::ldtk::FieldInstance;
    pub use bevy_ecs_ldtk::prelude::*;
    pub use bevy_inspector_egui::quick::WorldInspectorPlugin;
    pub use rand::prelude::SliceRandom;
    pub use rand::{thread_rng, Rng};
    pub use std::time::Duration;
}
use bevy::window::{WindowMode, WindowResolution};
use prelude::*;

fn main() {
    let mut app = App::new();

    // Bevy (or ecosystem) Plugins
    use bevy::asset::AssetMetaCheck;
    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: true,
                    title: "REDEATH".to_string(),
                    resolution: WindowResolution::new(WINDOW_WIDTH_f32, WINDOW_HEIGHT_f32),
                    mode: WindowMode::BorderlessFullscreen,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    )
    .add_plugins(WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Tab)));

    // Our plugins
    app.add_plugins(LayerPlugin)
        .add_plugins(RootPlugin)
        .add_plugins(StatePlugin);

    // Have fun!
    app.run();
}
