use crate::prelude::*;

pub mod bob;
pub mod chase;
mod coin;
mod egg;
mod egg_block;
mod oneoff;
mod replenish;

pub use bob::*;
pub use chase::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChaseSet;

pub(super) fn register_items(app: &mut App) {
    bob::register_bob(app);
    chase::register_chase(app);
    coin::register_coin(app);
    egg::register_egg(app);
    egg_block::register_egg_block(app);
    oneoff::register_oneoff(app);
    replenish::register_replenish(app);
}
