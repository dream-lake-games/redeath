use crate::prelude::*;

#[derive(Clone, Debug, Reflect)]
pub struct PauseImage {
    path: &'static str,
    zix: f32,
    render_layers: RenderLayers,
    pos: Pos,
}
impl PauseImage {
    pub fn new(path: &'static str) -> Self {
        Self {
            path,
            zix: 0.0,
            render_layers: MenuLayer::to_render_layers(),
            pos: Pos::default(),
        }
    }
    impl_with!(zix, f32);
    pub fn with_render_layers(mut self, rl: RenderLayers) -> Self {
        self.render_layers = rl;
        self
    }
    pub fn with_pos(mut self, pos: Pos) -> Self {
        self.pos = pos;
        self
    }
}
impl Component for PauseImage {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            let myself = world.get::<Self>(eid).unwrap();
            let rl = myself.render_layers.clone();
            let my_pos = myself.pos.clone();
            let ass = world.resource::<AssetServer>();
            let hand = ass.load(myself.path);
            let sprite_bund = SpriteBundle {
                texture: hand,
                transform: my_pos.to_spatial(myself.zix).transform,
                ..default()
            };
            world.commands().entity(eid).insert(sprite_bund);
            world.commands().entity(eid).insert(rl);
            world
                .commands()
                .entity(eid)
                .insert(Name::new("pause_image"));
            let parent = world.resource::<PauseRoot>().eid();
            world.commands().entity(eid).set_parent(parent);
        });
    }
}
