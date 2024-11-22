use crate::prelude::*;

pub mod bob;
mod coin;
mod egg;
mod egg_block;
mod oneoff;
mod replenish;

pub use bob::*;

pub(super) fn register_items(app: &mut App) {
    bob::register_bob(app);
    coin::register_coin(app);
    egg::register_egg(app);
    egg_block::register_egg_block(app);
    oneoff::register_oneoff(app);
    replenish::register_replenish(app);
}
