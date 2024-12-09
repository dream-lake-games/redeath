use crate::prelude::*;

mod autosave;
pub mod savefile_data;
pub mod savefile_get;
pub mod savefile_set;

use bevy_pkv::PkvStore;
pub use savefile_data::*;
pub use savefile_get::*;
pub use savefile_set::*;

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
    pub fn to_string(&self) -> String {
        match self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
        }
        .to_string()
    }
    pub fn all() -> Vec<Self> {
        vec![Self::A, Self::B, Self::C]
    }
}
impl AsRef<str> for SavefileKind {
    fn as_ref(&self) -> &str {
        match self {
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
        }
    }
}

#[derive(Resource)]
pub struct AllSavefiles {
    pub map: HashMap<SavefileKind, Savefile>,
}
impl AllSavefiles {
    fn load(store: &mut PkvStore) -> Self {
        let mut map = HashMap::new();
        for kind in SavefileKind::all() {
            let savefile = match store.get::<Savefile>(kind) {
                Ok(data) => data,
                Err(_) => {
                    let new_data = Savefile::default();
                    if let Err(e) = store.set(kind, &new_data) {
                        warn!("whoops couldn't set initial thing: {e:?}");
                    }
                    new_data
                }
            };
            map.insert(kind, savefile);
        }
        Self { map }
    }
}

#[derive(Resource)]
pub struct CurrentSavefileKind(pub SavefileKind);

#[derive(Resource)]
pub struct CurrentSavefile(pub Savefile);

pub(super) struct SavefilePlugin;
impl Plugin for SavefilePlugin {
    fn build(&self, app: &mut App) {
        reg_types!(app, SavefileKind, Savefile);

        let mut store = PkvStore::new("dreamlakegames", "redeath");
        app.insert_resource(AllSavefiles::load(&mut store));
        app.insert_resource(store);
        app.insert_resource(CurrentSavefileKind(SavefileKind::A));

        autosave::register_autosave(app);
        savefile_get::register_savefile_get(app);
        savefile_set::register_savefile_set(app);
    }
}
