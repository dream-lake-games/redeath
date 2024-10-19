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
]);

pub(super) fn register_effect_defns(app: &mut App) {
    app.insert_resource(SoundMults::default());
    debug_resource!(app, SoundMults);
}
