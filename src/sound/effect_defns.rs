use crate::prelude::*;

macro_rules! defn_effects {
    ([$($name:ident, $path:literal, $mult:literal,)*]) => {
        #[derive(Component, Clone, Copy, Debug, Reflect, std::hash::Hash, PartialEq, Eq)]
        pub enum SoundEffect {
            $($name,)*
        }
        impl SoundEffect {
            pub fn path(&self) -> String {
                match self {
                    $(Self::$name => $path.to_string(),)*
                }
            }
            pub fn mult(&self) -> f32 {
                match self {
                    $(Self::$name => $mult,)*
                }
            }
        }

        #[derive(Resource, Reflect)]
        pub struct SoundMults {
            pub map: HashMap<SoundEffect, f32>,
        }
        impl Default for SoundMults {
            fn default() -> Self {
                let mut map = HashMap::new();
                $(
                    map.insert(SoundEffect::$name, $mult);
                )*
                Self { map }
            }
        }
    };
}

defn_effects!([
    PlayerJump,
    "sound/player/jump.ogg",
    0.02,
    PlayerImpactRegular,
    "sound/player/impact_regular.ogg",
    0.1,
    PlayerWallSlide,
    "sound/player/slide.ogg",
    0.005,
    PlayerRunStep,
    "sound/player/jump.ogg",
    0.005,
    PlayerThunder,
    "sound/player/thunder.ogg",
    0.01,
    PlayerDeath1,
    "sound/player/death1.ogg",
    0.02,
    ReplenishBreak,
    "sound/replenish/break.ogg",
    0.025,
    ReplenishSpawn,
    "sound/replenish/spawn.ogg",
    0.02,
    MediumRain,
    "sound/world/8bit_medium_rain.ogg",
    0.02,
]);

// Persistent, looped sounds should have public structs from here for management
#[derive(Component)]
pub struct MediumRainSound;
#[derive(Bundle)]
pub struct MediumRainBundle {
    name: Name,
    rain: MediumRainSound,
    sound: SoundEffect,
    looped: LoopSound,
}
impl MediumRainBundle {
    pub fn new() -> Self {
        Self {
            name: Name::new("medium_rain"),
            rain: MediumRainSound,
            sound: SoundEffect::MediumRain,
            looped: LoopSound,
        }
    }
}

pub(super) fn register_effect_defns(app: &mut App) {
    app.insert_resource(SoundMults::default());
    debug_resource!(app, SoundMults);
}
