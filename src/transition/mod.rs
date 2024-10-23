use crate::prelude::*;

fn startup(mut commands: Commands, root: Res<TransitionRoot>) {
    commands
        .spawn((
            Name::new("TransitionAnim"),
            AnimMan::<TransitionAnim>::default().with_observe_state_changes(),
            SpatialBundle::from_transform(Transform {
                scale: Vec3::new(2.0, 2.0, 1.0),
                ..default()
            }),
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

pub enum TransitionPos {
    Screen(Vec2),
    World(Vec2),
}
impl Default for TransitionPos {
    fn default() -> Self {
        Self::Screen(default())
    }
}

#[derive(Event)]
pub struct StartTransition {
    pub to: MetaState,
    pub pos: TransitionPos,
}
impl StartTransition {
    pub fn to(to: MetaState) -> Self {
        Self {
            to,
            pos: TransitionPos::Screen(Vec2::ZERO),
        }
    }
    pub fn with_screen_pos(mut self, v: Vec2) -> Self {
        self.pos = TransitionPos::Screen(v);
        self
    }
    pub fn with_world_pos(mut self, v: Vec2) -> Self {
        self.pos = TransitionPos::World(v);
        self
    }
}

fn start_transition(
    trigger: Trigger<StartTransition>,
    current: Res<State<TransitionState>>,
    mut next: ResMut<NextState<TransitionState>>,
    meta_state: Res<State<MetaState>>,
    mut anim: Query<(&mut Transform, &mut AnimMan<TransitionAnim>)>,
    cam_pos: Query<&Pos, With<DynamicCamera>>,
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
    let (mut tran, mut anim) = anim.single_mut();
    match transition.pos {
        TransitionPos::Screen(v) => {
            tran.translation = v.extend(tran.translation.z);
        }
        TransitionPos::World(v) => {
            let cam_pos = cam_pos.single();
            tran.translation = (v - cam_pos.as_vec2()).extend(tran.translation.z);
        }
    }
    anim.reset_state(TransitionAnim::CircleOut);
}

#[derive(Event, Default)]
pub struct EndTransition {
    pos: TransitionPos,
}
impl EndTransition {
    pub fn center() -> Self {
        Self::default()
    }
    pub fn with_screen_pos(mut self, v: Vec2) -> Self {
        self.pos = TransitionPos::Screen(v);
        self
    }
    pub fn with_world_pos(mut self, v: Vec2) -> Self {
        self.pos = TransitionPos::World(v);
        self
    }
}

fn end_transition(
    trigger: Trigger<EndTransition>,
    current: Res<State<TransitionState>>,
    mut next: ResMut<NextState<TransitionState>>,
    mut anim: Query<(&mut Transform, &mut AnimMan<TransitionAnim>)>,
    cam_pos: Query<&Pos, With<DynamicCamera>>,
) {
    if !current.get().is_active() {
        warn!("Tried to end a transition, but none was active. Ignoring.");
        return;
    }
    next.set(TransitionState::default());
    let (mut tran, mut anim) = anim.single_mut();
    match trigger.event().pos {
        TransitionPos::Screen(v) => {
            tran.translation = v.extend(tran.translation.z);
        }
        TransitionPos::World(v) => {
            let cam_pos = cam_pos.single();
            tran.translation = (v - cam_pos.as_vec2()).extend(tran.translation.z);
        }
    }
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
