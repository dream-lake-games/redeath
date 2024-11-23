use crate::prelude::*;

mod tut;

pub(super) fn register_oneoff(app: &mut App) {
    tut::register_tut(app);
}
