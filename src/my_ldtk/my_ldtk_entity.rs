use std::marker::PhantomData;

use crate::prelude::*;

use super::MyLdtkChild;

pub trait MyLdtkEntity: Bundle {
    type Root: RootKind;
    fn from_ldtk(pos: Pos, fields: &HashMap<String, FieldValue>) -> Self;
}

#[derive(Component, Default)]
struct MyLdtkEntityWrapper<B: MyLdtkEntity> {
    _pd: PhantomComponent<B>,
    fields: HashMap<String, FieldValue>,
}
impl<B: MyLdtkEntity> LdtkEntity for MyLdtkEntityWrapper<B> {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _layer_instance: &LayerInstance,
        _tileset: Option<&Handle<Image>>,
        _tileset_definition: Option<&TilesetDefinition>,
        _asset_server: &AssetServer,
        _texture_atlases: &mut Assets<TextureAtlasLayout>,
    ) -> Self {
        Self {
            _pd: default(),
            fields: entity_instance
                .field_instances
                .clone()
                .into_iter()
                .map(|fi| (fi.identifier, fi.value))
                .collect(),
        }
    }
}

fn post_ldtk_entity_blessing<B: MyLdtkEntity>(
    mut commands: Commands,
    wrappers: Query<(Entity, &GlobalTransform, &MyLdtkEntityWrapper<B>)>,
    parents: Query<&Parent>,
    level_iids: Query<&LevelIid>,
    root: Res<B::Root>,
) {
    for (ldtk_eid, gt, wrapper) in &wrappers {
        // First get the level iid
        let granddad = parents
            .get(parents.get(ldtk_eid).expect("dad").get())
            .expect("granddad")
            .get();
        let level_iid = level_iids.get(granddad).expect("granddad has no leveliid");
        // Then spawn the thing
        let pos = Pos::new(gt.translation().x, gt.translation().y);
        let bund = B::from_ldtk(pos, &wrapper.fields);
        let child_eid = commands
            .spawn(bund)
            .insert(SpawnedLid {
                iid: level_iid.to_string(),
            })
            .set_parent(root.eid())
            .id();
        // Remember our child, but remove wrapper
        commands
            .entity(ldtk_eid)
            .insert(MyLdtkChild { child_eid })
            .remove::<MyLdtkEntityWrapper<B>>();
    }
}

pub struct MyLdtkEntityPlugin<B: MyLdtkEntity> {
    layer_id: &'static str,
    entity_id: &'static str,
    _pd: PhantomData<B>,
}
impl<B: MyLdtkEntity> MyLdtkEntityPlugin<B> {
    pub fn new(layer_id: &'static str, entity_id: &'static str) -> Self {
        Self {
            layer_id,
            entity_id,
            _pd: default(),
        }
    }
}
impl<B: MyLdtkEntity> Plugin for MyLdtkEntityPlugin<B> {
    fn build(&self, app: &mut App) {
        app.register_ldtk_entity_for_layer::<MyLdtkEntityWrapper<B>>(
            &self.layer_id,
            &self.entity_id,
        );
        app.add_systems(PreUpdate, post_ldtk_entity_blessing::<B>);
    }
}
