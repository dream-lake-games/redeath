macro_rules! reg_basic_fns {
    ($app:expr, $variant_name:ident) => {
        $app.add_systems(OnEnter(CutsceneState::$variant_name), on_enter);
        $app.add_systems(
            Update,
            update.run_if(in_state(CutsceneState::$variant_name)),
        );
        $app.add_systems(OnExit(CutsceneState::$variant_name), on_exit);
    };
}
pub(super) use reg_basic_fns;
