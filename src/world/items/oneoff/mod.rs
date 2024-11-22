use crate::prelude::*;

mod skelly_dash;

pub(super) fn register_oneoff(app: &mut App) {
    skelly_dash::register_skelly_dash(app);
}
