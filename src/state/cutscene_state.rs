use crate::prelude::*;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq, Reflect, States)]
pub enum CutsceneState {
    #[default]
    None,
    CanyonIntro,
    CanyonConfrontReaper,
    CanyonReaperConclusion,
}
