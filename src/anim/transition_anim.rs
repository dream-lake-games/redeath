use crate::prelude::*;

derive_anim!(
    pub enum TransitionAnim {
        #[default]
        #[file("clear.png")]
        #[size(1, 1)]
        #[render_layers(TransitionLayer)]
        Clear,
        // #[default]
        #[file("transitions/black.png")]
        #[size(360, 270)]
        #[render_layers(TransitionLayer)]
        Black,
        #[file("transitions/circle_in.png")]
        #[size(360, 270)]
        #[length(17)]
        #[next(Clear)]
        #[fps(32.0)]
        #[render_layers(TransitionLayer)]
        CircleIn,
        #[file("transitions/circle_out.png")]
        #[size(360, 270)]
        #[length(17)]
        #[fps(32.0)]
        #[next(Black)]
        #[render_layers(TransitionLayer)]
        CircleOut,
    }
);
type TransitionAnimPlugin = AnimDefnPlugin<TransitionAnim, AnimTimeRes>;

pub(super) fn register_transition_anim(app: &mut App) {
    app.add_plugins(TransitionAnimPlugin::default());
}
