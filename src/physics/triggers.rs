use crate::prelude::*;

use super::physics_maint::{impl_physics_comp, PhysicsComp, PhysicsComps, PhysicsCtrl};

// INTERESTING PART

#[derive(Clone, Copy, Debug, Reflect, PartialEq, Eq, std::hash::Hash)]
pub enum TriggerTxKind {
    Player,
    /// Specifically spikes so we can consolidate
    Spikes,
    /// Anything that should just kill the player instantly
    Kill,
    Replenish,
    Egg,
    Coin,
    Observe,
    Bounce,
    Bank,
}
#[derive(Clone, Copy, Debug, Reflect, PartialEq, Eq, std::hash::Hash)]
pub enum TriggerRxKind {
    Player,
    Firefly,
    WantStatic,
}

// PLUMBING
#[derive(Bundle, Reflect, Debug, Clone)]
pub struct TriggerTx {
    ctrl: TriggerTxCtrl,
    comps: PhysicsComps<TriggerTxComp>,
}
impl TriggerTx {
    pub fn new(data: Vec<(TriggerTxKind, HBox)>) -> Self {
        Self {
            ctrl: default(),
            comps: PhysicsComps::new(data),
        }
    }
    pub fn single(kind: TriggerTxKind, hbox: HBox) -> Self {
        Self::new(vec![(kind, hbox)])
    }
}
#[derive(Component, Clone, Debug, Reflect)]
pub struct TriggerTxComp {
    pub kind: TriggerTxKind,
    pub ctrl: Entity,
    pub hbox: HBox,
}
#[derive(Clone, Debug, Default, Reflect)]
pub struct TriggerTxCtrl {
    pub comps: Vec<Entity>,
    pub coll_keys: Vec<CollKey>,
}
impl_physics_comp!(TriggerTxKind, TriggerTxComp, TriggerTxCtrl);

#[derive(Bundle)]
pub struct TriggerRx {
    ctrl: TriggerRxCtrl,
    comps: PhysicsComps<TriggerRxComp>,
}
impl TriggerRx {
    pub fn new(data: Vec<(TriggerRxKind, HBox)>) -> Self {
        Self {
            ctrl: default(),
            comps: PhysicsComps::new(data),
        }
    }
    pub fn single(kind: TriggerRxKind, hbox: HBox) -> Self {
        Self::new(vec![(kind, hbox)])
    }
}
#[derive(Component, Clone, Debug, Reflect)]
pub struct TriggerRxComp {
    pub kind: TriggerRxKind,
    pub ctrl: Entity,
    pub hbox: HBox,
}
impl_physics_comp!(TriggerRxKind, TriggerRxComp, TriggerRxCtrl);
#[derive(Clone, Debug, Default, Reflect)]
pub struct TriggerRxCtrl {
    pub comps: Vec<Entity>,
    pub coll_keys: Vec<CollKey>,
}
