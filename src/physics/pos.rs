//! Pos functions as the source of truth for element translational placement.
//! It should be updated ONLY during `CollisionsSet`, which is a subset of `PhysicsSet`.
//! IPos is updated also in `CollisionsSet`, but is simply the rounded version of Pos.
//! Transforms are updated by looking at the IPos diffs, and adding.
//! This way we avoid global transform shenanigans.

use crate::prelude::*;

#[derive(Copy, Clone, Debug, Default, Reflect)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}
impl Component for Pos {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            let me = *world.get::<Self>(eid).expect("Couldn't get Pos after add");
            world.commands().entity(eid).insert(IPos::new(me));
        });
    }
}
impl Pos {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }
    pub fn as_ivec2(&self) -> IVec2 {
        IVec2::new(self.x.round() as i32, self.y.round() as i32)
    }
    pub fn to_transform(&self, zix: f32) -> Transform {
        Transform::from_translation(self.as_vec2().extend(zix))
    }
    pub fn translated(&self, offset: Vec2) -> Self {
        Self::new(self.x + offset.x, self.y + offset.y)
    }
}
impl std::ops::Add<Vec2> for Pos {
    type Output = Self;

    fn add(mut self, rhs: Vec2) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }
}
impl std::ops::AddAssign<Vec2> for Pos {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct IPos {
    pub cur: IVec2,
    pub last: IVec2,
}
impl IPos {
    fn new(pos: Pos) -> Self {
        let rounded = pos.as_ivec2();
        Self {
            cur: rounded,
            last: rounded,
        }
    }

    fn diff(&self) -> IVec2 {
        self.cur - self.last
    }
}

fn update_ipos(mut ents: Query<(&Pos, &mut IPos)>) {
    for (pos, mut ipos) in &mut ents {
        ipos.last = ipos.cur;
        ipos.cur = pos.as_ivec2();
    }
}

fn update_transforms(mut ents: Query<(&IPos, &mut Transform)>) {
    for (ipos, mut tran) in &mut ents {
        let diff3 = ipos.diff().as_vec2().extend(0.0);
        tran.translation += diff3;
    }
}

pub(super) fn register_pos(app: &mut App) {
    app.add_systems(
        Update,
        (update_ipos, update_transforms)
            .chain()
            .in_set(PhysicsSet)
            .in_set(super::CollSet)
            .in_set(super::PosSet)
            .run_if(in_state(PhysicsState::Active)),
    );
}
