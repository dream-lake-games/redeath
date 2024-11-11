macro_rules! add_common_systems {
    ($app:expr, $variant_name:ident) => {
        $app.add_systems(OnEnter(CutsceneState::$variant_name), on_enter);
        $app.add_systems(
            Update,
            update.run_if(in_state(CutsceneState::$variant_name)),
        );
        $app.add_systems(OnExit(CutsceneState::$variant_name), on_exit);
    };
}
pub(super) use add_common_systems;

macro_rules! add_puppet_init {
    ($app:expr, $variant_name:ident) => {
        $app.add_systems(
            OnEnter(PlayerMetaState::Puppet),
            init_puppet.run_if(in_state(CutsceneState::$variant_name)),
        );
    };
}
pub(super) use add_puppet_init;

macro_rules! add_update_systems {
    ($app:expr, $variant_name:ident, $($fun:ident$(,)?)+) => {
        $app.add_systems(
            Update,
            ($($fun,)+).run_if(in_state(CutsceneState::$variant_name)),
        );
    };
}
pub(super) use add_update_systems;

macro_rules! decl_cutscene_event {
    ($event_name:ident, $fun:item) => {
        #[derive(Event, Clone)]
        struct $event_name;
        $fun
    };
}
pub(super) use decl_cutscene_event;
