use crate::prelude::*;

pub mod bob;
mod egg;
mod replenish;

pub use bob::*;

pub(super) fn register_items(app: &mut App) {
    bob::register_bob(app);
    egg::register_egg(app);
    replenish::register_replenish(app);
}
