//! For things like keybindings, sound/music volume, etc...
//! we should remember what the player chooses.
//! This provides a common abstraction to create a resource
//! that is persistent.
//! This is nice because it means we can create these resources
//! in the same file as the relevant logic.

use crate::prelude::*;

/// Persistent resource-able
/// Yeah it's a terrible name
pub trait Persable: Resource + Reflect + Serialize + for<'de> Deserialize<'de> + Default {
    const KEY: &'static str;
}

#[derive(Resource, Reflect)]
pub struct Pers<R: Persable> {
    inner: R,
}
impl<R: Persable> Pers<R> {
    pub fn load() -> Self {
        let mut store = PkvStore::new("dreamlakegames", "redeath");
        let inner = match store.get::<R>(R::KEY) {
            Ok(data) => data,
            Err(_) => {
                let inner_inner = R::default();
                if let Err(e) = store.set(R::KEY, &inner_inner) {
                    warn!("hmm couldn't set initial state for {}: {e:?}", R::KEY);
                }
                inner_inner
            }
        };
        Self { inner }
    }

    // Gets the current resource value from memory. Does NOT load from disk.
    pub fn get(&self) -> &R {
        &self.inner
    }

    /// Updates the resource but DOES NOT write it to disk
    pub fn set(&mut self, res: R) {
        self.inner = res;
    }

    /// TODO: I know bevy has special support for doing things like file IO, but I haven't
    /// learned it yet. This (imo) is a mild problem across the game
    /// I should learn how to do that
    /// For now call this sparingly
    pub fn save(&self, store: &mut PkvStore) {
        if let Err(e) = store.set(R::KEY, &self.inner) {
            warn!("couldn't save {}: {e:?}", R::KEY);
        }
    }
}
