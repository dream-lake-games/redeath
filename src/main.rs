pub mod anim;
pub mod bgfg;
pub mod camera;
pub mod consts;
pub mod convo;
mod cutscene;
pub mod debug;
pub mod input;
pub mod layer;
pub mod lazy;
pub mod light;
pub mod math;
pub mod menu;
pub mod misc;
pub mod my_ldtk;
pub mod palette;
pub mod pause;
pub mod persistent_resource;
pub mod physics;
pub mod player;
pub mod reaper;
pub mod root;
pub mod savefile;
pub mod sound;
pub mod state;
pub mod transition;
pub mod types;
pub mod world;

pub mod prelude {
    pub use super::{
        anim::*, bgfg::*, camera::*, consts::*, convo::*, input::*, layer::*, lazy::*, light::*,
        math::*, my_ldtk::*, palette::*, persistent_resource::*, physics::*, player::*, reaper::*,
        root::*, savefile::*, sound::*, state::*, transition::*, types::*, world::*,
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
    pub use bevy::{sprite::Anchor, text::TextBounds};
    pub use bevy_2delight_anims::prelude::*;
    pub use bevy_ecs_ldtk::ldtk::FieldInstance;
    pub use bevy_ecs_ldtk::prelude::*;
    pub use bevy_ecs_tilemap::prelude::*;
    pub use bevy_inspector_egui::quick::WorldInspectorPlugin;
    pub use bevy_pkv::PkvStore;
    pub use rand::prelude::SliceRandom;
    pub use rand::{thread_rng, Rng};
    pub use serde::{Deserialize, Serialize};
    pub use std::{ops::Range, time::Duration};
}
use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    window::{WindowMode, WindowResolution},
};
use menu::MenuPlugin;
use misc::MiscPlugin;
use pause::PausePlugin;
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
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),
    );
    app.add_plugins(FrameTimeDiagnosticsPlugin::default());
    // .add_plugins(LogDiagnosticsPlugin::default());

    #[cfg(debug_assertions)]
    {
        app.add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Tab)),
        );
    }

    // Our plugins
    app.add_plugins(CameraPlugin)
        .add_plugins(BgFgPlugin)
        .add_plugins(ConvoPlugin)
        .add_plugins(cutscene::CutscenePlugin)
        .add_plugins(debug::DebugPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(LayerPlugin)
        .add_plugins(LightPlugin)
        .add_plugins(MenuPlugin)
        .add_plugins(MiscPlugin)
        .add_plugins(MyAnimPlugin)
        .add_plugins(MyLdtkPlugin)
        .add_plugins(PalettePlugin)
        .add_plugins(PausePlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(ReaperPlugin)
        .add_plugins(RootPlugin)
        .add_plugins(SavefilePlugin)
        .add_plugins(SoundPlugin)
        .add_plugins(StatePlugin)
        .add_plugins(TransitionPlugin)
        .add_plugins(WorldPlugin);

    // Have fun!
    app.run();
}
