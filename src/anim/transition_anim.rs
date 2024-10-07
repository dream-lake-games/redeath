use crate::prelude::*;

derive_anim!(
    pub enum TransitionAnim {
        #[default]
        #[file("clear.png")]
        #[size(1, 1)]
        #[render_layers(MenuLayer)]
        Clear,
        // #[default]
        #[file("transitions/black.png")]
        #[size(360, 270)]
        #[render_layers(MenuLayer)]
        Black,
        #[file("transitions/circle_in.png")]
        #[size(360, 270)]
        #[length(8)]
        #[next(Clear)]
        #[render_layers(MenuLayer)]
        CircleIn,
        #[file("transitions/circle_out.png")]
        #[size(360, 270)]
        #[length(8)]
        #[next(Black)]
        #[render_layers(MenuLayer)]
        CircleOut,
    }
);
pub type TransitionAnimPlugin = AnimDefnPlugin<TransitionAnim, AnimTimeRes>;
