use crate::prelude::*;

mod transition_anim;

pub use transition_anim::TransitionAnim;
use transition_anim::TransitionAnimPlugin;

#[derive(Resource, Clone, Debug, Default, Reflect)]
pub struct AnimTimeRes {
    class_map: HashMap<AnimTimeClass, f32>,
}
impl AnimTimeProvider for AnimTimeRes {
    fn get_delta(&self, class: AnimTimeClass) -> f32 {
        *self.class_map.get(&class).unwrap_or(&0.0)
    }
}

fn drive_anim_time_res(
    mut anim_time: ResMut<AnimTimeRes>,
    time: Res<Time>,
    paused: Option<Res<State<PauseState>>>,
) {
    anim_time.class_map.insert(
        ANIM_TIME_BULLET,
        paused.as_ref().map(|_| 0.0).unwrap_or(time.delta_seconds()),
    );
    anim_time.class_map.insert(
        ANIM_TIME_REAL,
        paused.as_ref().map(|_| 0.0).unwrap_or(time.delta_seconds()),
    );
    anim_time
        .class_map
        .insert(ANIM_TIME_BULLET_ALWAYS, time.delta_seconds());
    anim_time
        .class_map
        .insert(ANIM_TIME_REAL_ALWAYS, time.delta_seconds());
}

pub(super) struct MyAnimPlugin;
impl Plugin for MyAnimPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AnimTimeRes::default());
        app.add_plugins(
            AnimPlugin::<AnimTimeRes>::default()
                .with_default_fps(DEFAULT_ANIMATION_FPS)
                .with_default_render_layers(MainLayer::to_render_layers())
                .with_default_time_class(ANIM_TIME_BULLET),
        );
        app.add_systems(PostUpdate, drive_anim_time_res.before(AnimSet));

        app.add_plugins(TransitionAnimPlugin::default());
    }
}
