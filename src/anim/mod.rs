use crate::prelude::*;

mod light_anim;
mod menu_anim;
mod player_anim;
mod transition_anim;

pub use light_anim::*;
pub use menu_anim::*;
pub use player_anim::*;
pub use transition_anim::TransitionAnim;

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
    bullet_time: Res<BulletTime>,
    time: Res<Time>,
    paused: Res<State<PauseState>>,
) {
    let paused_delta = match paused.get() {
        PauseState::Paused => 0.0,
        PauseState::Unpaused => 1.0,
    };

    anim_time
        .class_map
        .insert(ANIM_TIME_BULLET, paused_delta * bullet_time.delta_seconds());
    anim_time
        .class_map
        .insert(ANIM_TIME_REAL, paused_delta * time.delta_seconds());
    anim_time
        .class_map
        .insert(ANIM_TIME_BULLET_ALWAYS, bullet_time.delta_seconds());
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

        light_anim::register_light_anim(app);
        menu_anim::register_menu_anim(app);
        player_anim::register_player_anim(app);
        transition_anim::register_transition_anim(app);
    }
}
