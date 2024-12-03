use crate::prelude::*;

derive_anim!(
    pub enum ReaperAnim {
        #[default]
        #[file("reaper/idle.png")]
        #[size(28, 28)]
        #[length(4)]
        #[offset(0.0, 2.0)]
        #[fps(8.0)]
        #[render_layers(StaticLayer)]
        Idle,
        #[file("reaper/appear_idle.png")]
        #[size(28, 28)]
        #[length(7)]
        #[offset(0.0, -2.0)]
        #[next(Idle)]
        #[render_layers(StaticLayer)]
        AppearIdle,
        #[file("reaper/idle_disappear.png")]
        #[size(28, 28)]
        #[length(7)]
        #[offset(0.0, -2.0)]
        #[next(Despawn)]
        #[render_layers(StaticLayer)]
        IdleDisappear,
        #[file("reaper/hover.png")]
        #[size(28, 28)]
        #[length(5)]
        #[fps(8.0)]
        #[render_layers(StaticLayer)]
        Hover,
        #[file("reaper/appear_hover.png")]
        #[size(28, 28)]
        #[length(7)]
        #[next(Hover)]
        #[render_layers(StaticLayer)]
        AppearHover,
        #[file("reaper/hover_disappear.png")]
        #[size(28, 28)]
        #[length(7)]
        #[next(None)]
        #[render_layers(StaticLayer)]
        HoverDisappear,
        #[file("reaper/charge.png")]
        #[size(28, 28)]
        #[length(13)]
        #[next(Throw)]
        #[render_layers(StaticLayer)]
        Charge,
        #[file("reaper/throw.png")]
        #[size(28, 28)]
        #[length(9)]
        #[next(Hover)]
        #[render_layers(StaticLayer)]
        Throw,
        #[file("clear.png")]
        #[size(1, 1)]
        None,
    }
);
type ReaperAnimPlugin = AnimDefnPlugin<ReaperAnim, AnimTimeRes>;

derive_anim!(
    pub enum ScytheAnim {
        #[default]
        #[file("reaper/scythe/out.png")]
        #[size(28, 28)]
        #[length(8)]
        #[render_layers(StaticLayer)]
        Out,
        #[file("reaper/scythe/become_ball.png")]
        #[size(28, 28)]
        #[length(4)]
        #[next(Ball)]
        #[render_layers(StaticLayer)]
        BecomeBall,
        #[file("reaper/scythe/ball.png")]
        #[size(28, 28)]
        #[length(6)]
        #[render_layers(StaticLayer)]
        Ball,
        #[file("reaper/scythe/ball_pop.png")]
        #[size(28, 28)]
        #[length(5)]
        #[render_layers(StaticLayer)]
        BallPop,
    }
);
type ScytheAnimPlugin = AnimDefnPlugin<ScytheAnim, AnimTimeRes>;

derive_anim!(
    pub enum ScythePartAnim {
        #[default]
        #[file("reaper/scythe/part0.png")]
        #[size(28, 28)]
        #[length(5)]
        #[render_layers(StaticLayer)]
        #[next(Despawn)]
        Part0,
        #[file("reaper/scythe/part1.png")]
        #[size(28, 28)]
        #[length(5)]
        #[render_layers(StaticLayer)]
        #[next(Despawn)]
        Part1,
        #[file("reaper/scythe/part2.png")]
        #[size(28, 28)]
        #[length(5)]
        #[render_layers(StaticLayer)]
        #[next(Despawn)]
        Part2,
        #[file("reaper/scythe/part3.png")]
        #[size(28, 28)]
        #[length(5)]
        #[render_layers(StaticLayer)]
        #[next(Despawn)]
        Part3,
        #[file("reaper/scythe/part4.png")]
        #[size(28, 28)]
        #[length(5)]
        #[render_layers(StaticLayer)]
        #[next(Despawn)]
        Part4,
        #[file("reaper/scythe/part5.png")]
        #[size(28, 28)]
        #[length(5)]
        #[render_layers(StaticLayer)]
        #[next(Despawn)]
        Part5,
        #[file("reaper/scythe/part6.png")]
        #[size(28, 28)]
        #[length(5)]
        #[render_layers(StaticLayer)]
        #[next(Despawn)]
        Part6,
        #[file("reaper/scythe/part7.png")]
        #[size(28, 28)]
        #[length(5)]
        #[render_layers(StaticLayer)]
        #[next(Despawn)]
        Part7,
    }
);
type ScythePartAnimPlugin = AnimDefnPlugin<ScythePartAnim, AnimTimeRes>;

pub(super) fn register_reaper_anim(app: &mut App) {
    app.add_plugins(ReaperAnimPlugin::default());
    app.add_plugins(ScytheAnimPlugin::default());
    app.add_plugins(ScythePartAnimPlugin::default());
}
