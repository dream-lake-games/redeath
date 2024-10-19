use crate::prelude::*;

derive_anim!(
    pub enum PlayerAnim {
        #[default]
        #[file("player/lenny_stand.png")]
        #[size(24, 24)]
        Stand,
        #[file("player/lenny_squat.png")]
        #[size(24, 24)]
        Squat,
        #[file("player/lenny_run.png")]
        #[size(24, 24)]
        #[length(14)]
        #[fps(24.0)]
        Run,
        #[file("player/lenny_jump.png")]
        #[size(24, 24)]
        #[length(5)]
        #[next(MidAir)]
        #[fps(24.0)]
        Jump,
        #[file("player/lenny_air.png")]
        #[size(24, 24)]
        #[length(4)]
        MidAir,
        #[file("player/lenny_land.png")]
        #[size(24, 24)]
        #[length(2)]
        #[next(Stand)]
        Land,
    }
);
type PlayerAnimPlugin = AnimDefnPlugin<PlayerAnim, AnimTimeRes>;

pub(super) fn register_player_anim(app: &mut App) {
    app.add_plugins(PlayerAnimPlugin::default());
}
