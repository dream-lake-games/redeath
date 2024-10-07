use std::marker::PhantomData;

use crate::prelude::*;

use super::MyLdtkChild;

pub trait MyLdtkIntCell: Bundle {
    type Root: RootKind;
    type RenderLayers: Into<RenderLayers>;
    fn from_ldtk(pos: Pos, value: i32) -> Self;
}

#[derive(Component, Default)]
struct MyLdtkIntCellWrapper<B: MyLdtkIntCell> {
    _pd: PhantomComponent<B>,
    value: i32,
}
impl<B: MyLdtkIntCell> LdtkIntCell for MyLdtkIntCellWrapper<B> {
    fn bundle_int_cell(int_grid_cell: IntGridCell, _layer_instance: &LayerInstance) -> Self {
        Self {
            _pd: default(),
            value: int_grid_cell.value,
        }
    }
}

fn post_ldtk_int_cell_blessing<B: MyLdtkIntCell>(
    mut commands: Commands,
    wrappers: Query<(Entity, &GlobalTransform, &MyLdtkIntCellWrapper<B>)>,
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
        let bund = B::from_ldtk(pos, wrapper.value);
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
            .remove::<MyLdtkIntCellWrapper<B>>();
    }
}

pub struct MyLdtkIntCellPlugin<B: MyLdtkIntCell> {
    layer_id: &'static str,
    value: i32,
    _pd: PhantomData<B>,
}
impl<B: MyLdtkIntCell> MyLdtkIntCellPlugin<B> {
    pub fn new(layer_id: &'static str, value: i32) -> Self {
        Self {
            layer_id,
            value,
            _pd: default(),
        }
    }
}
impl<B: MyLdtkIntCell> Plugin for MyLdtkIntCellPlugin<B> {
    fn build(&self, app: &mut App) {
        app.register_ldtk_int_cell_for_layer::<MyLdtkIntCellWrapper<B>>(&self.layer_id, self.value);
        app.add_systems(PreUpdate, post_ldtk_int_cell_blessing::<B>);
    }
}
