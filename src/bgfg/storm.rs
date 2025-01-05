use crate::prelude::*;

use super::parallax::{ParallaxX, ParallaxY};

#[derive(Default, Resource)]
pub struct StormManager {
    last_show_rain: bool,
    show_rain: bool,
}
impl StormManager {
    pub fn show_rain(&mut self) {
        self.show_rain = true;
    }
    pub fn hide_rain(&mut self) {
        self.show_rain = false;
    }
}

#[derive(Bundle)]
struct RainTallBundle {
    name: Name,
    transform: Transform,
    visibility: Visibility,
    anim: AnimMan<RainTallAnim>,
    parallax_x: ParallaxX,
    parallax_y: ParallaxY,
}
impl RainTallBundle {
    fn new() -> Self {
        Self {
            name: Name::new("rain_tall"),
            transform: Pos::default().to_transform(ZIX_RAIN),
            visibility: Visibility::Inherited,
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
    transform: Transform,
    visibility: Visibility,
    parallax_x: ParallaxX,
    anim: AnimMan<LightningAnim>,
}
impl LightningBundle {
    fn new() -> Self {
        Self {
            name: Name::new("lightning"),
            transform: Pos::default().to_transform(20.0),
            visibility: Visibility::Inherited,
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
    mut manager: ResMut<StormManager>,
    mut commands: Commands,
    root: Res<BgFgRoot>,
    existing_q: Query<Entity, With<AnimMan<RainTallAnim>>>,
) {
    match (manager.last_show_rain, manager.show_rain) {
        (false, true) => {
            //shutup rust
            commands.spawn(RainTallBundle::new()).set_parent(root.eid());
            manager.last_show_rain = true;
        }
        (true, false) => {
            for eid in &existing_q {
                commands.entity(eid).despawn_recursive();
                manager.last_show_rain = false;
            }
        }
        _ => {}
    }
    if existing_q.is_empty() {
        manager.show_rain = false;
        manager.last_show_rain = false;
    }
}

pub(super) fn register_rain(app: &mut App) {
    app.insert_resource(StormManager::default());

    app.add_systems(Update, update_rain);
}
