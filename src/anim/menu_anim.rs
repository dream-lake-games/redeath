use crate::prelude::*;

derive_anim!(
    #[time_class(ANIM_TIME_REAL_ALWAYS)]
    pub enum SavefileButtonAnim {
        #[default]
        #[file("menu/savefile_button_idle.png")]
        #[size(180, 48)]
        #[render_layers(MenuLayer)]
        Idle,
        #[file("menu/savefile_button_activate.png")]
        #[size(180, 48)]
        #[length(2)]
        #[render_layers(MenuLayer)]
        #[next(Active)]
        Activate,
        #[file("menu/savefile_button_active.png")]
        #[size(180, 48)]
        #[render_layers(MenuLayer)]
        Active,
    }
);
type SavefileButtonAnimPlugin = AnimDefnPlugin<SavefileButtonAnim, AnimTimeRes>;

derive_anim!(
    #[time_class(ANIM_TIME_REAL_ALWAYS)]
    pub enum PauseButtonAnim {
        #[default]
        #[file("menu/pause/pause_button_idle.png")]
        #[size(180, 48)]
        #[render_layers(MenuLayer)]
        Idle,
        #[file("menu/pause/pause_button_activate.png")]
        #[size(180, 48)]
        #[length(2)]
        #[render_layers(MenuLayer)]
        #[next(Active)]
        Activate,
        #[file("menu/pause/pause_button_active.png")]
        #[size(180, 48)]
        #[render_layers(MenuLayer)]
        Active,
    }
);
type PauseButtonAnimPlugin = AnimDefnPlugin<PauseButtonAnim, AnimTimeRes>;

derive_anim!(
    pub enum OverworldContinueAnim {
        #[default]
        #[file("menu/overworld_assets/continue_disabled.png")]
        #[size(64, 64)]
        #[render_layers(PaletteLayer)]
        Disabled,
        #[file("menu/overworld_assets/continue_closed.png")]
        #[size(64, 64)]
        Closed,
        #[file("menu/overworld_assets/continue_opening.png")]
        #[size(64, 64)]
        #[length(3)]
        #[next(Open)]
        Opening,
        #[file("menu/overworld_assets/continue_open.png")]
        #[size(64, 64)]
        Open,
        #[file("menu/overworld_assets/continue_closing.png")]
        #[size(64, 64)]
        #[length(3)]
        #[next(Closed)]
        Closing,
    }
);
type ContinueAnimPlugin = AnimDefnPlugin<OverworldContinueAnim, AnimTimeRes>;

derive_anim!(
    pub enum OverworldRestartAnim {
        #[default]
        #[file("menu/overworld_assets/restart_idle.png")]
        #[size(64, 64)]
        Idle,
        #[file("menu/overworld_assets/restart_active.png")]
        #[size(64, 64)]
        #[length(6)]
        Active,
    }
);
type RestartAnimPlugin = AnimDefnPlugin<OverworldRestartAnim, AnimTimeRes>;

pub(super) fn register_menu_anim(app: &mut App) {
    app.add_plugins(SavefileButtonAnimPlugin::default());
    app.add_plugins(PauseButtonAnimPlugin::default());
    app.add_plugins(ContinueAnimPlugin::default());
    app.add_plugins(RestartAnimPlugin::default());
}
