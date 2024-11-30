use crate::prelude::*;

mod animals_anim;
mod common_platform_anim;
mod items_anim;
mod light_anim;
mod menu_anim;
mod player_anim;
mod reaper_anim;
mod star_anim;
mod storm_anim;
mod transition_anim;
mod tut_anim;

pub use animals_anim::*;
pub use common_platform_anim::*;
pub use items_anim::*;
pub use light_anim::*;
pub use menu_anim::*;
pub use player_anim::*;
pub use reaper_anim::*;
pub use star_anim::*;
pub use storm_anim::*;
pub use transition_anim::TransitionAnim;
pub use tut_anim::*;

#[derive(Bundle)]
pub struct EphemeralAnim<StateMachine: AnimStateMachine> {
    anim: AnimMan<StateMachine>,
    spat: SpatialBundle,
}
impl<StateMachine: AnimStateMachine> EphemeralAnim<StateMachine> {
    pub fn new(anim: StateMachine, flip_x: bool, pos: Pos, zix: f32) -> Self {
        Self {
            anim: AnimMan::new(anim).with_flip_x(flip_x),
            spat: pos.to_spatial(zix),
        }
    }
}

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
    level_scroll_kind: Res<State<LevelScrollStateKind>>,
) {
    let paused_delta = match paused.get() {
        PauseState::Paused => 0.0,
        PauseState::Unpaused => 1.0,
    };
    let level_scroll_delta = match level_scroll_kind.get() {
        LevelScrollStateKind::None => 1.0,
        LevelScrollStateKind::Some => 0.0,
    };

    anim_time.class_map.insert(
        ANIM_TIME_BULLET,
        paused_delta * level_scroll_delta * bullet_time.delta_seconds(),
    );
    anim_time.class_map.insert(
        ANIM_TIME_REAL,
        paused_delta * level_scroll_delta * time.delta_seconds(),
    );
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

        animals_anim::register_animals_anim(app);
        common_platform_anim::register_common_platform_anim(app);
        items_anim::register_items_anim(app);
        light_anim::register_light_anim(app);
        menu_anim::register_menu_anim(app);
        player_anim::register_player_anim(app);
        storm_anim::register_storm_anim(app);
        star_anim::register_star_anim(app);
        transition_anim::register_transition_anim(app);
        tut_anim::register_tut_anim(app);
        reaper_anim::register_reaper_anim(app);
    }
}
