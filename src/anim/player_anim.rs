use crate::prelude::*;

derive_anim!(
    pub enum PlayerAnim {
        #[default]
        #[file("player/movement/stand.png")]
        #[size(22, 22)]
        Stand,
        #[file("player/movement/squat.png")]
        #[size(22, 22)]
        Squat,
        #[file("player/movement/run.png")]
        #[size(22, 22)]
        #[length(5)]
        Run,
        #[file("player/movement/jump.png")]
        #[size(22, 22)]
        #[length(2)]
        #[next(AirUp)]
        Jump,
        #[file("player/movement/air_up.png")]
        #[size(22, 22)]
        AirUp,
        #[file("player/movement/air_up_exhausted.png")]
        #[size(22, 22)]
        AirUpExhausted,
        #[file("player/movement/air_down.png")]
        #[size(22, 22)]
        AirDown,
        #[file("player/movement/air_down_exhausted.png")]
        #[size(22, 22)]
        AirDownExhausted,
        #[file("player/movement/land.png")]
        #[size(22, 22)]
        #[length(2)]
        #[next(Stand)]
        Land,
        #[file("player/movement/wall_slide.png")]
        #[size(22, 22)]
        WallSlide,
        #[file("player/movement/wall_slide_exhausted.png")]
        #[size(22, 22)]
        WallSlideExhausted,
        #[file("player/movement/wall_jump.png")]
        #[size(22, 22)]
        #[next(AirUp)]
        WallJump,
        #[file("player/movement/wall_jump_exhausted.png")]
        #[size(22, 22)]
        #[next(AirUpExhausted)]
        WallJumpExhausted,
        #[file("player/movement/dash.png")]
        #[size(22, 22)]
        Dash,
        #[file("player/movement/wall_push.png")]
        #[size(22, 22)]
        #[length(8)]
        WallPush,
        #[file("player/movement/death1.png")]
        #[size(22, 22)]
        #[length(9)]
        #[next(AfterDeath)]
        Death,
        #[file("clear.png")]
        #[size(1, 1)]
        AfterDeath,
    }
);
type PlayerAnimPlugin = AnimDefnPlugin<PlayerAnim, AnimTimeRes>;

derive_anim!(
    pub enum JumpSmokeAnim {
        #[default]
        #[file("player/smoke/jump_regular1.png")]
        #[size(16, 16)]
        #[length(8)]
        #[next(Despawn)]
        Regular1,
        #[file("player/smoke/jump_wall1.png")]
        #[size(16, 16)]
        #[length(7)]
        #[next(Despawn)]
        Wall1,
        #[file("player/smoke/jump_land1.png")]
        #[size(16, 16)]
        #[length(4)]
        #[next(Despawn)]
        Land1,
    }
);
type JumpSmokeAnimPlugin = AnimDefnPlugin<JumpSmokeAnim, AnimTimeRes>;

derive_anim!(
    pub enum WallSlideSmokeAnim {
        #[default]
        #[file("player/smoke/wall_slide1.png")]
        #[size(16, 16)]
        #[length(4)]
        #[next(Despawn)]
        WallSlide1,
    }
);
type WallSlideSmokeAnimPlugin = AnimDefnPlugin<WallSlideSmokeAnim, AnimTimeRes>;

derive_anim!(
    pub enum RunSmokeAnim {
        #[default]
        #[file("player/smoke/run1.png")]
        #[size(16, 16)]
        #[length(5)]
        #[next(Despawn)]
        Run1,
    }
);
type RunSmokeAnimPlugin = AnimDefnPlugin<RunSmokeAnim, AnimTimeRes>;

derive_anim!(
    pub enum DashDieAnim {
        #[default]
        #[file("player/movement/dash_die.png")]
        #[size(22, 22)]
        #[length(10)]
        #[next(Despawn)]
        DashDie,
    }
);
type DashDieAnimPlugin = AnimDefnPlugin<DashDieAnim, AnimTimeRes>;

derive_anim!(
    pub enum DashFadeAnim {
        #[default]
        #[file("player/movement/dash_fade.png")]
        #[size(22, 22)]
        #[length(3)]
        #[next(Despawn)]
        #[render_layers(PaletteLayer)]
        #[fps(6.0)]
        DashFade,
    }
);
type DashFadeAnimPlugin = AnimDefnPlugin<DashFadeAnim, AnimTimeRes>;

pub(super) fn register_player_anim(app: &mut App) {
    app.add_plugins(PlayerAnimPlugin::default());
    app.add_plugins(JumpSmokeAnimPlugin::default());
    app.add_plugins(WallSlideSmokeAnimPlugin::default());
    app.add_plugins(RunSmokeAnimPlugin::default());
    app.add_plugins(DashDieAnimPlugin::default());
    app.add_plugins(DashFadeAnimPlugin::default());
}
