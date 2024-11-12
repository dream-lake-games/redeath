use crate::prelude::*;

mod replenish;

pub(super) fn register_items(app: &mut App) {
    replenish::register_replenish(app);
}
