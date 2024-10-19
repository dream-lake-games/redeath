use crate::prelude::*;

derive_anim!(
    pub enum PlayerAnim {
        #[default]
        #[file("player/stand.png")]
        #[size(22, 22)]
        Stand,
        #[file("player/squat.png")]
        #[size(22, 22)]
        Squat,
        #[file("player/run.png")]
        #[size(22, 22)]
        #[length(5)]
        Run,
        #[file("player/jump.png")]
        #[size(22, 22)]
        #[length(2)]
        #[next(AirUp)]
        Jump,
        #[file("player/air_up.png")]
        #[size(22, 22)]
        AirUp,
        #[file("player/air_down.png")]
        #[size(22, 22)]
        AirDown,
        #[file("player/land.png")]
        #[size(22, 22)]
        #[length(2)]
        #[next(Stand)]
        Land,
        #[file("player/wall_slide.png")]
        #[size(22, 22)]
        WallSlide,
        #[file("player/wall_jump.png")]
        #[size(22, 22)]
        WallJump,
        #[file("player/dash.png")]
        #[size(22, 22)]
        Dash,
    }
);
type PlayerAnimPlugin = AnimDefnPlugin<PlayerAnim, AnimTimeRes>;

derive_anim!(
    pub enum JumpAnim {
        #[default]
        #[file("smoke/jump/regular1.png")]
        #[size(16, 16)]
        #[length(8)]
        #[next(Despawn)]
        Regular1,
        #[file("smoke/jump/wall1.png")]
        #[size(16, 16)]
        #[length(7)]
        #[next(Despawn)]
        Wall1,
    }
);
type JumpAnimPlugin = AnimDefnPlugin<JumpAnim, AnimTimeRes>;

pub(super) fn register_player_anim(app: &mut App) {
    app.add_plugins(PlayerAnimPlugin::default());
    app.add_plugins(JumpAnimPlugin::default());
}
