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
        Idle,
        #[file("reaper/appear.png")]
        #[size(28, 28)]
        #[length(7)]
        #[offset(0.0, -2.0)]
        #[next(Idle)]
        Appear,
        #[file("reaper/disappear.png")]
        #[size(28, 28)]
        #[length(7)]
        #[offset(0.0, -2.0)]
        #[next(Despawn)]
        Disappear,
    }
);
type ReaperAnimPlugin = AnimDefnPlugin<ReaperAnim, AnimTimeRes>;

pub(super) fn register_reaper_anim(app: &mut App) {
    app.add_plugins(ReaperAnimPlugin::default());
}
