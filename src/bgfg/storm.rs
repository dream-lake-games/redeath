use crate::prelude::*;

use super::parallax::{ParallaxX, ParallaxY};

#[derive(Default, Resource)]
pub struct RainManager {
    last_shown: bool,
    shown: bool,
}
impl RainManager {
    pub fn show(&mut self) {
        self.shown = true;
    }
    pub fn hide(&mut self) {
        self.shown = false;
    }
}

#[derive(Bundle)]
struct RainTallBundle {
    name: Name,
    spatial: SpatialBundle,
    anim: AnimMan<RainTallAnim>,
    parallax_x: ParallaxX,
    parallax_y: ParallaxY,
}
impl RainTallBundle {
    fn new() -> Self {
        Self {
            name: Name::new("rain_tall"),
            spatial: Pos::default().to_spatial(ZIX_RAIN),
            anim: AnimMan::default()
                .with_rep_x(SCREEN_WIDTH)
                .with_rep_y(SCREEN_HEIGHT),
            parallax_x: ParallaxX::new(0.5, SCREEN_WIDTH_f32),
            parallax_y: ParallaxY::new(0.5, SCREEN_HEIGHT_f32),
        }
    }
}

pub struct Lightning;

#[derive(Bundle)]
struct LightningBundle {
    name: Name,
    spatial: SpatialBundle,
    parallax_x: ParallaxX,
    anim: AnimMan<LightningAnim>,
}
impl LightningBundle {
    fn new() -> Self {
        Self {
            name: Name::new("lightning"),
            spatial: Pos::default().to_spatial(20.0),
            parallax_x: ParallaxX::new(0.13, SCREEN_WIDTH_f32),
            anim: AnimMan::new(LightningAnim::random()).with_flip_x(thread_rng().gen_bool(0.5)),
        }
    }
}

impl Component for Lightning {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            let parent = world.resource::<WorldDetailRoot>().eid();
            world
                .commands()
                .entity(eid)
                .insert(LightningBundle::new())
                .set_parent(parent);
        });
    }
}

fn update_rain(
    mut manager: ResMut<RainManager>,
    mut commands: Commands,
    root: Res<WorldDetailRoot>,
    existing_q: Query<Entity, With<AnimMan<RainTallAnim>>>,
) {
    match (manager.last_shown, manager.shown) {
        (false, true) => {
            //shutup rust
            commands.spawn(RainTallBundle::new()).set_parent(root.eid());
            manager.last_shown = true;
        }
        (true, false) => {
            for eid in &existing_q {
                commands.entity(eid).despawn_recursive();
                manager.last_shown = false;
            }
        }
        _ => {}
    }
}

pub(super) fn register_rain(app: &mut App) {
    app.insert_resource(RainManager::default());

    app.add_systems(Update, update_rain);
}
