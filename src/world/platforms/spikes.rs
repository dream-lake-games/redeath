use crate::prelude::*;

#[derive(Component)]
struct Spikes;

#[derive(Bundle)]
struct SpikesBundle {
    name: Name,
    pos: Pos,
    trigger_tx: TriggerTx,
    spikes: Spikes,
}
impl MyLdtkIntCell for SpikesBundle {
    type Root = PlatformRoot;
    type RenderLayers = StaticLayer;
    type LeftoverRenderLayers = MainAmbienceLayer;
    fn from_ldtk(pos: Pos, _value: i32) -> Self {
        Self {
            name: Name::new("spikes"),
            pos,
            trigger_tx: TriggerTx::single(TriggerTxKind::Spikes, HBox::new(6, 6)),
            spikes: Spikes,
        }
    }
}

#[derive(Component, Debug)]
struct SpikeGroupBase {
    row_group: i32,
    col_group: i32,
}
impl SpikeGroupBase {
    fn from_tilemap_index(index: i32) -> Self {
        Self {
            row_group: (index / 32) / 2,
            col_group: (index % 32) / 2,
        }
    }
    fn to_tilemap_index(&self, offset: &SpikeGroupOffset) -> i32 {
        let row_base = self.row_group * 64 + offset.row_offset * 32;
        let col_base = self.col_group * 2 + offset.col_offset;
        row_base + col_base
    }
}

#[derive(Component, Debug)]
struct SpikeGroupOffset {
    row_offset: i32,
    col_offset: i32,
}
impl SpikeGroupOffset {
    fn from_tilemap_index(index: i32) -> Self {
        Self {
            row_offset: (index / 32) % 2,
            col_offset: (index % 32) % 2,
        }
    }
    fn rotate(&mut self) {
        if self.col_offset == 0 {
            self.col_offset = (self.col_offset + 1) % 2;
        } else {
            self.row_offset = (self.row_offset + 1) % 2;
        }
    }
}

fn add_spike_group_info(
    query: Query<
        (Entity, &MyLdtkParent),
        (
            With<Spikes>,
            Without<SpikeGroupBase>,
            Without<SpikeGroupOffset>,
        ),
    >,
    ixs: Query<&TileTextureIndex>,
    mut commands: Commands,
) {
    for (eid, parent) in &query {
        let ix = ixs.get(parent.get()).unwrap();
        let base = SpikeGroupBase::from_tilemap_index(ix.0 as i32);
        let offset = SpikeGroupOffset::from_tilemap_index(ix.0 as i32);
        println!("\nbase: {base:?}");
        println!("offset: {offset:?}");
        println!(
            "compare: {} -> {}\n",
            ix.0 as i32,
            base.to_tilemap_index(&offset)
        );
        commands.entity(eid).insert(base);
        commands.entity(eid).insert(offset);
    }
}

fn rotate_spike_group_offsets(mut offsets: Query<&mut SpikeGroupOffset>) {
    for mut offset in &mut offsets {
        if thread_rng().gen_bool(0.01) {
            offset.rotate();
        }
    }
}

fn update_tilemap_ixs(
    ents: Query<(&MyLdtkParent, &SpikeGroupBase, &SpikeGroupOffset)>,
    mut ixs: Query<&mut TileTextureIndex>,
) {
    for (parent, base, offset) in &ents {
        let mut ix = ixs.get_mut(parent.get()).unwrap();
        ix.0 = base.to_tilemap_index(offset) as u32;
    }
}

pub(super) fn register_spikes(app: &mut App) {
    app.add_plugins(MyLdtkIntCellPlugin::<SpikesBundle>::single("Spikes", 1));

    app.add_systems(
        Update,
        (
            add_spike_group_info,
            rotate_spike_group_offsets,
            update_tilemap_ixs,
        )
            .run_if(in_state(MetaStateKind::World)),
    );
}
