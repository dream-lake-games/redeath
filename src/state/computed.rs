use crate::prelude::*;

impl ComputedStates for LevelState {
    type SourceStates = WorldState;

    fn compute(sources: Self::SourceStates) -> Option<Self> {
        Some(sources.level_state)
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect)]
pub enum PhysicsState {
    Active,
    Inactive,
}
impl ComputedStates for PhysicsState {
    type SourceStates = (LevelState, Option<PauseState>, Option<TransState>);
    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            (LevelState { .. }, None, None) => Some(PhysicsState::Active),
            _ => Some(PhysicsState::Inactive),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect)]
pub struct TransActiveState;
impl ComputedStates for TransActiveState {
    type SourceStates = TransState;
    fn compute(_: Self::SourceStates) -> Option<Self> {
        Some(Self)
    }
}

impl ComputedStates for WorldKind {
    type SourceStates = (Option<WorldLoadingState>, Option<WorldState>);
    fn compute(sources: Self::SourceStates) -> Option<Self> {
        match sources {
            (Some(WorldLoadingState { kind }), None) => Some(kind),
            (None, Some(WorldState { kind, .. })) => Some(kind),
            _ => None,
        }
    }
}

macro_rules! impl_core_computed_state {
    ($var:ident, $type:ty) => {
        impl ComputedStates for $type {
            type SourceStates = MetaState;
            fn compute(sources: Self::SourceStates) -> Option<Self> {
                match sources {
                    MetaState::$var(thing) => Some(thing),
                    _ => None,
                }
            }
        }
        impl CoreState for $type {
            fn to_meta_state(self) -> MetaState {
                MetaState::$var(self)
            }
            fn from_meta_state(meta: &MetaState) -> Option<Self> {
                match meta {
                    MetaState::$var(thing) => Some(thing.clone()),
                    _ => None,
                }
            }
        }
    };
}
pub(crate) use impl_core_computed_state;

macro_rules! impl_kind_computed_state {
    ($base_state:ty, $($kind:ident$(,)?)*) => {
        paste::paste! {
            #[derive(Clone, Debug, Eq, Hash, PartialEq, Reflect)]
            pub enum [<$base_state Kind>] {
                $($kind,)*
            }
            impl ComputedStates for [<$base_state Kind>] {
                type SourceStates = $base_state;
                fn compute(sources: Self::SourceStates) -> Option<Self> {
                    match sources {
                        $(
                            $base_state::$kind { .. } => Some(Self::$kind),
                        )*
                    }
                }
            }
        }
    };
}
pub(crate) use impl_kind_computed_state;
