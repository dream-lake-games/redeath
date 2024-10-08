use crate::prelude::*;

#[derive(Component, Clone, Copy, Debug, Reflect, PartialEq, Eq, Hash)]
pub enum SavefileKind {
    A,
    B,
    C,
}
impl SavefileKind {
    pub fn next(&self) -> Option<Self> {
        match self {
            Self::A => Some(Self::B),
            Self::B => Some(Self::C),
            Self::C => None,
        }
    }
    pub fn prev(&self) -> Option<Self> {
        match self {
            Self::A => None,
            Self::B => Some(Self::A),
            Self::C => Some(Self::B),
        }
    }
}

#[derive(Clone, Debug, Reflect)]
pub struct SavefileData {}

#[derive(Resource, Clone, Debug, Reflect)]
pub struct Savefile {
    kind: SavefileKind,
    data: SavefileData,
}
impl Savefile {
    pub fn change_me(kind: SavefileKind) -> Self {
        Self {
            kind,
            data: SavefileData {},
        }
    }
    impl_get_copy!(kind, SavefileKind);
    impl_get_clone!(data, SavefileData);
}

pub(super) struct SavefilePlugin;
impl Plugin for SavefilePlugin {
    fn build(&self, app: &mut App) {
        reg_types!(app, SavefileKind, SavefileData, Savefile);
    }
}
