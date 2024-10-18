use crate::prelude::*;

use super::physics_maint::{impl_physics_comp, PhysicsComp, PhysicsComps, PhysicsCtrl};

// INTERESTING PART

#[derive(Clone, Copy, Debug, Reflect, PartialEq, Eq, std::hash::Hash)]
pub enum StaticTxKind {
    /// Standard solid thing. Stops stuff
    Solid,
    /// A platform that only stops things that are moving down
    PassUp,
}
#[derive(Clone, Copy, Debug, Reflect, PartialEq, Eq, std::hash::Hash)]
pub enum StaticRxKind {
    /// Pushes the rx ctrl out of tx comps, sets vel to zero along plane of intersection
    Default,
    /// Observes collisions but does nothing to respond
    Observe,
}

// PLUMBING
#[derive(Bundle, Debug, Clone, Reflect)]
pub struct StaticTx {
    ctrl: StaticTxCtrl,
    comps: PhysicsComps<StaticTxComp>,
}
impl StaticTx {
    pub fn new(data: Vec<(StaticTxKind, HBox)>) -> Self {
        Self {
            ctrl: default(),
            comps: PhysicsComps::new(data),
        }
    }
    pub fn single(kind: StaticTxKind, hbox: HBox) -> Self {
        Self::new(vec![(kind, hbox)])
    }
}
#[derive(Component, Clone, Debug, Reflect)]
pub struct StaticTxComp {
    pub kind: StaticTxKind,
    pub ctrl: Entity,
    pub hbox: HBox,
}
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct StaticTxCtrl {
    pub comps: Vec<Entity>,
    pub coll_keys: Vec<CollKey>,
}
impl_physics_comp!(StaticTxKind, StaticTxComp, StaticTxCtrl);

#[derive(Bundle)]
pub struct StaticRx {
    ctrl: StaticRxCtrl,
    comps: PhysicsComps<StaticRxComp>,
}
impl StaticRx {
    pub fn new(data: Vec<(StaticRxKind, HBox)>) -> Self {
        Self {
            ctrl: default(),
            comps: PhysicsComps::new(data),
        }
    }
    pub fn single(kind: StaticRxKind, hbox: HBox) -> Self {
        Self::new(vec![(kind, hbox)])
    }
}
#[derive(Component, Clone, Debug, Reflect)]
pub struct StaticRxComp {
    pub kind: StaticRxKind,
    pub ctrl: Entity,
    pub hbox: HBox,
}
impl_physics_comp!(StaticRxKind, StaticRxComp, StaticRxCtrl);
#[derive(Component, Clone, Debug, Default, Reflect)]
pub struct StaticRxCtrl {
    pub comps: Vec<Entity>,
    pub coll_keys: Vec<CollKey>,
}
