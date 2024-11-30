use crate::prelude::*;

derive_anim!(
    pub enum CruncherAnim {
        #[default]
        #[file("environment/cruncher/idle.png")]
        #[size(16, 40)]
        #[render_layers(StaticLayer)]
        Idle,
        #[file("environment/cruncher/crunch.png")]
        #[size(16, 40)]
        #[length(15)]
        #[render_layers(StaticLayer)]
        #[next(Idle)]
        Crunch,
    }
);
type CruncherAnimPlugin = AnimDefnPlugin<CruncherAnim, AnimTimeRes>;

pub(super) fn register_cruncher_anim(app: &mut App) {
    app.add_plugins(CruncherAnimPlugin::default());
}
