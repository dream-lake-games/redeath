use crate::prelude::*;

fn startup(mut commands: Commands, root: Res<TransitionRoot>) {
    commands
        .spawn((
            Name::new("TransitionAnim"),
            AnimMan::<TransitionAnim>::default().with_observe_state_changes(),
            SpatialBundle::default(),
        ))
        .set_parent(root.eid());
}

fn anim_state_change(
    trigger: Trigger<AnimStateChange<TransitionAnim>>,
    transition: Res<State<TransitionState>>,
    mut next_meta: ResMut<NextState<MetaState>>,
) {
    let state_change = trigger.event();
    if state_change.next == TransitionAnim::Black {
        let Some(to) = transition.get().entering.clone() else {
            warn!("transition to black but no active transition...");
            return;
        };
        next_meta.set(to);
    }
}

#[derive(Event)]
pub struct StartTransition {
    pub to: MetaState,
}
impl StartTransition {
    pub fn to(to: MetaState) -> Self {
        Self { to }
    }
}

fn start_transition(
    trigger: Trigger<StartTransition>,
    current: Res<State<TransitionState>>,
    mut next: ResMut<NextState<TransitionState>>,
    meta_state: Res<State<MetaState>>,
    mut anim: Query<&mut AnimMan<TransitionAnim>>,
) {
    if current.get().is_active() {
        warn!("Trying to do two transitions at once. Ignoring.");
        return;
    }
    let transition = trigger.event();
    next.set(TransitionState {
        exiting: Some(meta_state.get().clone()),
        entering: Some(transition.to.clone()),
    });
    let mut anim = anim.single_mut();
    anim.reset_state(TransitionAnim::CircleOut);
}

#[derive(Event)]
pub struct EndTransition;

fn end_transition(
    _trigger: Trigger<EndTransition>,
    current: Res<State<TransitionState>>,
    mut next: ResMut<NextState<TransitionState>>,
    mut anim: Query<&mut AnimMan<TransitionAnim>>,
) {
    if !current.get().is_active() {
        warn!("Tried to end a transition, but none was active. Ignoring.");
        return;
    }
    next.set(TransitionState::default());
    let mut anim = anim.single_mut();
    anim.reset_state(TransitionAnim::CircleIn);
}

pub(super) struct TransitionPlugin;
impl Plugin for TransitionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup.after(RootInit));
        app.observe(start_transition);
        app.observe(end_transition);
        app.observe(anim_state_change);
    }
}
