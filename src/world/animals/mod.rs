use crate::prelude::*;

mod firefly;

pub(super) fn register_animals(app: &mut App) {
    firefly::register_firefly(app);
}
