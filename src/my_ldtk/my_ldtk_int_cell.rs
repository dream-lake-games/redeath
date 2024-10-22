use std::marker::PhantomData;

use bevy::utils::hashbrown::HashMap;
use bevy_ecs_tilemap::FrustumCulling;

use crate::prelude::*;

use super::MyLdtkChild;

pub trait MyLdtkIntCell: Bundle {
    type Root: RootKind;
    type RenderLayers: Layer;
    type LeftoverRenderLayers: Layer;
    fn from_ldtk(pos: Pos, value: i32) -> Self;
}

#[derive(Component, Default)]
struct MyLdtkIntCellWrapper<B: MyLdtkIntCell> {
    _pd: PhantomComponent<B>,
    value: i32,
    _blocker: BlockMyLdtkLoad,
}
impl<B: MyLdtkIntCell> LdtkIntCell for MyLdtkIntCellWrapper<B> {
    fn bundle_int_cell(int_grid_cell: IntGridCell, _layer_instance: &LayerInstance) -> Self {
        Self {
            _pd: default(),
            value: int_grid_cell.value,
            _blocker: BlockMyLdtkLoad::ticks(10),
        }
    }
}

struct DuplicateTilemapRoot {
    render_layers: RenderLayers,
    older_brother: Entity,
}
fn duplicate_tilemap_root(
    trigger: DuplicateTilemapRoot,
    tilemap_q: &Query<(
        &Name,
        &Parent,
        &Transform,
        &FrustumCulling,
        &Handle<StandardTilemapMaterial>,
        &TileStorage,
        &TilemapGridSize,
        &TilemapRenderSettings,
        &TilemapSize,
        &TilemapSpacing,
        &TilemapTexture,
        &TilemapTileSize,
        &TilemapType,
    )>,
    commands: &mut Commands,
) -> Entity {
    let DuplicateTilemapRoot {
        render_layers,
        older_brother,
    } = trigger;
    let (
        name,
        parent,
        transform,
        fc,
        hand,
        storage,
        grid_size,
        render_settings,
        size,
        spacing,
        texture,
        tile_size,
        type_,
    ) = tilemap_q.get(older_brother).unwrap();
    commands
        .spawn((
            name.clone(),
            SpatialBundle::from_transform(transform.clone()),
            fc.clone(),
            hand.clone(),
            storage.clone(),
            grid_size.clone(),
            render_settings.clone(),
            size.clone(),
            spacing.clone(),
            texture.clone(),
            tile_size.clone(),
            type_.clone(),
        ))
        .insert(render_layers)
        .set_parent(parent.get())
        .id()
}

fn post_ldtk_int_cell_blessing<B: MyLdtkIntCell>(
    mut commands: Commands,
    mut wrappers: Query<(Entity, &GlobalTransform, &MyLdtkIntCellWrapper<B>)>,
    // Nasty hack
    mut tile_flip: Query<&mut TileFlip>,
    parents: Query<&Parent>,
    children_q: Query<&Children>,
    level_iids: Query<&LevelIid>,
    root: Res<B::Root>,
    tilemap_q: Query<(
        &Name,
        &Parent,
        &Transform,
        &FrustumCulling,
        &Handle<StandardTilemapMaterial>,
        &TileStorage,
        &TilemapGridSize,
        &TilemapRenderSettings,
        &TilemapSize,
        &TilemapSpacing,
        &TilemapTexture,
        &TilemapTileSize,
        &TilemapType,
    )>,
) {
    let mut new_dad_map: HashMap<i32, Entity> = default();
    let mut needs_cleanup = HashSet::<Entity>::default();
    for (ldtk_eid, gt, wrapper) in &mut wrappers {
        // First get the level iid
        let dad = parents.get(ldtk_eid).expect("dad").get();
        let granddad = parents.get(dad).expect("granddad").get();
        let level_iid = level_iids.get(granddad).expect("granddad has no leveliid");
        // Then spawn the thing
        let pos = Pos::new(gt.translation().x, gt.translation().y);
        let bund = B::from_ldtk(pos, wrapper.value);
        let child_eid = commands
            .spawn(bund)
            .insert(SpawnedLid {
                iid: level_iid.to_string(),
            })
            .set_parent(root.eid())
            .id();
        // Holy fuck this api is cursed
        let key = B::RenderLayers::to_i32();
        if let Some(new_dad) = new_dad_map.get(&key).cloned() {
            commands.entity(ldtk_eid).set_parent(new_dad);
            commands.entity(ldtk_eid).insert(TilemapId(new_dad));
            let mut flip = tile_flip.get_mut(ldtk_eid).unwrap();
            flip.x = flip.x;
        } else {
            let new_dad = duplicate_tilemap_root(
                DuplicateTilemapRoot {
                    render_layers: B::RenderLayers::to_render_layers(),
                    older_brother: dad,
                },
                &tilemap_q,
                &mut commands,
            );
            new_dad_map.insert(key, new_dad);
        }
        // Remember our child, but remove wrapper
        commands
            .entity(ldtk_eid)
            .insert(MyLdtkChild { child_eid })
            .remove::<MyLdtkIntCellWrapper<B>>();
        needs_cleanup.insert(dad);
    }
    if !needs_cleanup.is_empty() {
        let first = needs_cleanup.iter().next().cloned().unwrap();
        let cleanup_dad = duplicate_tilemap_root(
            DuplicateTilemapRoot {
                render_layers: B::LeftoverRenderLayers::to_render_layers(),
                older_brother: first,
            },
            &tilemap_q,
            &mut commands,
        );
        if let Ok(children) = children_q.get(first) {
            for child in children {
                if !wrappers.contains(*child) {
                    commands.entity(*child).set_parent(cleanup_dad);
                    commands.entity(*child).insert(TilemapId(cleanup_dad));
                    let mut flip = tile_flip.get_mut(*child).unwrap();
                    flip.x = flip.x;
                }
            }
        }
    }
}

pub struct MyLdtkIntCellPlugin<B: MyLdtkIntCell> {
    layer_id: &'static str,
    values: Vec<i32>,
    _pd: PhantomData<B>,
}
impl<B: MyLdtkIntCell> MyLdtkIntCellPlugin<B> {
    pub fn single(layer_id: &'static str, value: i32) -> Self {
        Self {
            layer_id,
            values: vec![value],
            _pd: default(),
        }
    }
    pub fn multiple(layer_id: &'static str, values: Vec<i32>) -> Self {
        Self {
            layer_id,
            values,
            _pd: default(),
        }
    }
}
impl<B: MyLdtkIntCell> Plugin for MyLdtkIntCellPlugin<B> {
    fn build(&self, app: &mut App) {
        for value in &self.values {
            app.register_ldtk_int_cell_for_layer::<MyLdtkIntCellWrapper<B>>(&self.layer_id, *value);
        }
        app.add_systems(PreUpdate, post_ldtk_int_cell_blessing::<B>);
    }
}
