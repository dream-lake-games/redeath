use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Reflect, serde::Serialize, serde::Deserialize)]
pub struct Savefile {
    pub current_world: WorldKind,
    pub world_savefiles: HashMap<WorldKind, WorldSavefileData>,
    pub speedrun_timer_us: u128,
    pub num_deaths: u64,
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
            speedrun_timer_us: 0,
            num_deaths: 0,
        }
    }
}
impl Savefile {
    pub fn is_new_savefile(&self) -> bool {
        self.current_world == WorldKind::Canyon
            && self.world_savefiles[&WorldKind::Canyon].current_run == None
            && !self.world_savefiles[&WorldKind::Canyon].ever_finished
    }

    /// In the `current_world`, get level that we should go to for continuing a run
    pub fn get_current_run_lid(&self) -> String {
        self.world_savefiles[&self.current_world]
            .current_run
            .clone()
            .unwrap_or(self.get_new_run_lid())
    }

    /// In the `current_world`, get level that we should go to for continuing a run OR NONE
    pub fn get_current_run_lid_opt(&self) -> Option<String> {
        self.world_savefiles[&self.current_world]
            .current_run
            .clone()
    }

    /// In the `current_world`, get the level that we should go to for restarting a run
    pub fn get_new_run_lid(&self) -> String {
        match self.current_world {
            WorldKind::Canyon => "d32f7850-73f0-11ef-ab29-c106faf0247d".to_string(),
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
