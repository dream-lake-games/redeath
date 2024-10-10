use crate::prelude::*;

mod scratch;
mod spawn;

pub(super) struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        scratch::register_scratch(app);
        spawn::register_spawn(app);
    }
}
