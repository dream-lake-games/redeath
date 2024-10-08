pub mod anim;
pub mod consts;
pub mod debug;
pub mod input;
pub mod layer;
pub mod lazy;
pub mod menu;
pub mod my_ldtk;
pub mod palette;
pub mod physics;
pub mod root;
pub mod savefile;
pub mod state;
pub mod transition;
pub mod types;

pub mod prelude {
    pub use super::{
        anim::*, consts::*, input::*, layer::*, lazy::*, my_ldtk::*, palette::*, physics::*,
        root::*, savefile::*, state::*, transition::*, types::*,
    };
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
    pub use bevy_2delight_anims::prelude::*;
    pub use bevy_ecs_ldtk::ldtk::FieldInstance;
    pub use bevy_ecs_ldtk::prelude::*;
    pub use bevy_inspector_egui::quick::WorldInspectorPlugin;
    pub use rand::prelude::SliceRandom;
    pub use rand::{thread_rng, Rng};
    pub use std::time::Duration;
}
use bevy::window::{WindowMode, WindowResolution};
use menu::MenuPlugin;
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
    app.add_plugins(debug::DebugPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(LayerPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(MyAnimPlugin)
        .add_plugins(MyLdtkPlugin)
        .add_plugins(PalettePlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(RootPlugin)
        .add_plugins(SavefilePlugin)
        .add_plugins(StatePlugin)
        .add_plugins(TransitionPlugin);

    // Have fun!
    app.run();
}
