use crate::prelude::*;

#[derive(Component)]
struct DeadTreeEntity;
impl SimpleSpriteEntity for DeadTreeEntity {
    const NAME: &'static str = "dead_tree";
    const PATH: &'static str = "environment/plants/dead_tree.png";
    const ZIX: f32 = 0.0;
    type Root = WorldDetailRoot;
    type RenderLayers = MainLayer;
}

pub(super) fn register_plants(app: &mut App) {
    app.add_plugins(
        MyLdtkEntityPlugin::<SimpleSpriteBundle<DeadTreeEntity>>::new("Entities", "DeadTree"),
    );
}
