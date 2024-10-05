use crate::prelude::*;

derive_anim!(
    pub enum TransitionAnim {
        #[default]
        #[file("clear.png")]
        #[size(1, 1)]
        Clear,
        // #[default]
        #[file("transitions/black.png")]
        #[size(360, 270)]
        Black,
        #[file("transitions/circle_in.png")]
        #[size(360, 270)]
        #[length(8)]
        #[next(Clear)]
        CircleIn,
        #[file("transitions/circle_out.png")]
        #[size(360, 270)]
        #[length(8)]
        #[next(Black)]
        CircleOut,
    }
);
pub type TransitionAnimPlugin = AnimDefnPlugin<TransitionAnim, AnimTimeRes>;
