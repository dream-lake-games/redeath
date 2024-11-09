use crate::prelude::*;

mod draw_hitboxes;

fn debug_startup(mut gizmo_config_store: ResMut<GizmoConfigStore>, mut _commands: Commands) {
    // Gizmo config
    let (config, _) = gizmo_config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 2.0;
    config.render_layers = StaticLayer::to_render_layers();
}

fn _spawn_light(pos: Pos, commands: &mut Commands) {
    commands.spawn((
        Name::new("debug_light"),
        pos,
        pos.to_spatial(0.0 + thread_rng().gen_range(0.0..1.0)),
        Light::new(PlayerLightAnim::Static128),
    ));
}

fn debug_update(mut commands: Commands, keyboard: Res<ButtonInput<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::KeyU) {
        commands.trigger(StartConvoLake::Hello);
    }
}

fn enter_state() {
    println!("entered state");
}

fn my_in_state() {
    if thread_rng().gen_bool(0.01) {
        println!("in state rate limited");
    }
}

fn exit_state() {
    println!("exited state");
}

fn start_trigger(trigger: Trigger<ConvoLakeStarted>) {
    println!("start_trigger: {:?}", trigger.event());
}
fn end_trigger(trigger: Trigger<ConvoLakeEnded>) {
    println!("end_trigger: {:?}", trigger.event());
}

pub(super) struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, debug_startup.after(RootInit));
        app.add_systems(Update, debug_update);
        draw_hitboxes::register_draw_hitboxes(app);

        app.observe(start_trigger);
        app.observe(end_trigger);

        app.add_systems(OnEnter(ConvoMetaState::Some), enter_state);
        app.add_systems(Update, my_in_state.run_if(in_state(ConvoMetaState::Some)));
        app.add_systems(OnEnter(ConvoMetaState::None), exit_state);
    }
}
