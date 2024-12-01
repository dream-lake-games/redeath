use crate::prelude::*;

derive_anim!(
    #[time_class(ANIM_TIME_REAL)]
    pub enum ReaperAnim {
        #[default]
        #[file("reaper/idle.png")]
        #[size(28, 28)]
        #[length(4)]
        #[offset(0.0, 2.0)]
        #[fps(8.0)]
        #[render_layers(StaticLayer)]
        Idle,
        #[file("reaper/appear_idle.png")]
        #[size(28, 28)]
        #[length(7)]
        #[offset(0.0, -2.0)]
        #[next(Idle)]
        #[render_layers(StaticLayer)]
        AppearIdle,
        #[file("reaper/idle_disappear.png")]
        #[size(28, 28)]
        #[length(7)]
        #[offset(0.0, -2.0)]
        #[next(Despawn)]
        #[render_layers(StaticLayer)]
        IdleDisappear,
        #[file("reaper/hover.png")]
        #[size(28, 28)]
        #[length(5)]
        #[offset(0.0, 2.0)]
        #[fps(8.0)]
        #[render_layers(StaticLayer)]
        Hover,
        #[file("reaper/appear_hover.png")]
        #[size(28, 28)]
        #[length(7)]
        #[offset(0.0, -2.0)]
        #[next(Hover)]
        #[render_layers(StaticLayer)]
        AppearHover,
        #[file("reaper/hover_disappear.png")]
        #[size(28, 28)]
        #[length(7)]
        #[offset(0.0, -2.0)]
        #[next(None)]
        #[render_layers(StaticLayer)]
        HoverDisappear,
        #[file("clear.png")]
        #[size(1, 1)]
        None,
    }
);
type ReaperAnimPlugin = AnimDefnPlugin<ReaperAnim, AnimTimeRes>;

pub(super) fn register_reaper_anim(app: &mut App) {
    app.add_plugins(ReaperAnimPlugin::default());
}
