use crate::prelude::*;

mod canyon_confront_reaper;
mod canyon_intro;
mod canyon_reaper_conclusion;
mod cutscene_macros;
mod cutscene_trigger;

mod cutlude {
    use crate::prelude::*;

    pub(super) use super::cutscene_macros::*;

    #[derive(Clone)]
    pub(super) struct DoInSeconds<E: Event + Clone> {
        seconds: f32,
        event: E,
    }
    impl<E: Event + Clone> DoInSeconds<E> {
        pub(super) fn new(event: E, seconds: f32) -> Self {
            Self { seconds, event }
        }
    }
    impl<E: Event + Clone> Component for DoInSeconds<E> {
        const STORAGE_TYPE: StorageType = StorageType::SparseSet;
        fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
            hooks.on_add(|mut world, eid, _| {
                let time_left = world.get::<Self>(eid).expect("myself").seconds;
                let parent = world.resource::<CutsceneRoot>().eid();
                world
                    .commands()
                    .entity(eid)
                    .insert(super::DoInSecondsInner { time_left })
                    .set_parent(parent);
            });
            hooks.on_remove(|mut world, eid, _| {
                let event = world.get::<Self>(eid).expect("myself").event.clone();
                world.commands().trigger(event);
            });
        }
    }
}

fn clear_root(mut commands: Commands, root: Res<CutsceneRoot>) {
    if let Some(mut commands) = commands.get_entity(root.eid()) {
        commands.despawn_descendants();
    }
}

fn force_puppet(
    meta_state: Res<State<MetaState>>,
    mut next_meta_state: ResMut<NextState<MetaState>>,
) {
    let MetaState::World(mut world_state) = meta_state.get().clone() else {
        return;
    };
    world_state.player_meta_state = PlayerMetaState::Puppet;
    next_meta_state.set(world_state.to_meta_state());
}

#[derive(Component)]
struct DoInSecondsInner {
    time_left: f32,
}
fn drive_do_in_seconds(
    mut inners: Query<(Entity, &mut DoInSecondsInner)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (eid, mut inner) in &mut inners {
        inner.time_left -= time.delta_secs();
        if inner.time_left <= 0.0 {
            commands.entity(eid).despawn_recursive();
        }
    }
}

pub(super) struct CutscenePlugin;
impl Plugin for CutscenePlugin {
    fn build(&self, app: &mut App) {
        // Clear the root and force puppet always
        app.add_systems(OnExit(CutsceneState::None), (clear_root, force_puppet));
        app.add_systems(OnEnter(CutsceneState::None), clear_root);
        // Drive do in seconds
        app.add_systems(
            Update,
            drive_do_in_seconds.run_if(not(in_state(CutsceneState::None))),
        );

        cutscene_trigger::register_cutscene_trigger(app);

        canyon_intro::register_canyon_intro(app);
        canyon_confront_reaper::register_canyon_confront_reaper(app);
        canyon_reaper_conclusion::register_canyon_reaper_conclusion(app);
    }
}
