use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Reflect, serde::Serialize, serde::Deserialize)]
pub struct Savefile {
    pub current_world: WorldKind,
    pub world_savefiles: HashMap<WorldKind, WorldSavefileData>,
}
impl Default for Savefile {
    fn default() -> Self {
        Self {
            current_world: WorldKind::Canyon,
            world_savefiles: HashMap::from_iter(
                WorldKind::all()
                    .into_iter()
                    .map(|kind| (kind, WorldSavefileData::default())),
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Reflect, serde::Serialize, serde::Deserialize, Default)]
pub struct WorldSavefileData {
    /// Set containing the iids of the strings you've collected in this world
    pub collected_coins: HashSet<String>,
    /// Have you ever finished this world?
    pub ever_finished: bool,
    /// In your current run (if a current run exists), what lid are you on?
    pub current_run: Option<String>,
}
