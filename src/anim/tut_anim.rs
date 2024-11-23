use crate::prelude::*;

derive_anim!(
    pub enum SkellyDashAnim {
        #[default]
        #[file("environment/tut/skelly_dash.png")]
        #[size(32, 24)]
        #[render_layers(StaticLayer)]
        Spin,
    }
);
type SkellyDashAnimPlugin = AnimDefnPlugin<SkellyDashAnim, AnimTimeRes>;

derive_anim!(
    pub enum DashDrawingAnim {
        #[default]
        #[file("environment/tut/dash_drawing_right.png")]
        #[size(40, 40)]
        #[render_layers(StaticLayer)]
        Right,
        #[file("environment/tut/dash_drawing_up.png")]
        #[size(40, 40)]
        #[render_layers(StaticLayer)]
        Up,
        #[file("environment/tut/dash_drawing_diagonal.png")]
        #[size(40, 40)]
        #[render_layers(StaticLayer)]
        Diagonal,
    }
);
type DashDrawingAnimPlugin = AnimDefnPlugin<DashDrawingAnim, AnimTimeRes>;

pub(super) fn register_tut_anim(app: &mut App) {
    app.add_plugins(SkellyDashAnimPlugin::default());
    app.add_plugins(DashDrawingAnimPlugin::default());
}
