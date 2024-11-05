use std::cmp::Ordering;

use crate::prelude::*;

#[derive(Component, Debug, Clone, Reflect)]
pub enum MyLdtkConsolidateKind {
    /// For consolidating things which are a single solid 8x8 hitbox
    Solid8x8,
}

#[derive(Clone)]
struct ConsolidationEntry {
    eid: Entity,
    comp_eid: Entity,
}
fn consolidate_int_cells(
    _trigger: Trigger<LevelChangeEvent>,
    consolidating: Query<(Entity, &MyLdtkConsolidateKind, &StaticTxCtrl, &Pos)>,
    stx_comps: Query<&StaticTxComp>,
    mut commands: Commands,
) {
    fn get_key(pos: &Pos) -> (i32, i32) {
        ((pos.x / 8.0) as i32, (pos.y / 8.0) as i32)
    }

    let mut solid8x8_keys: Vec<(i32, i32)> = Vec::with_capacity(consolidating.iter().count());
    let mut solid8x8_entries: HashMap<(i32, i32), ConsolidationEntry> = default();
    for (eid, kind, ctrl, pos) in &consolidating {
        let key = get_key(pos);
        match kind {
            MyLdtkConsolidateKind::Solid8x8 => {
                debug_assert!(ctrl.comps.len() == 1);
                let comp = stx_comps.get(ctrl.comps[0]).unwrap();
                debug_assert!(comp.kind == StaticTxKind::Solid);
                debug_assert!(comp.hbox.get_offset() == Vec2::ZERO);
                debug_assert!(comp.hbox.get_size() == UVec2::new(8, 8));
                solid8x8_keys.push(key);
                solid8x8_entries.insert(
                    key,
                    ConsolidationEntry {
                        eid,
                        comp_eid: ctrl.comps[0],
                    },
                );
            }
        }
        commands.entity(eid).remove::<MyLdtkConsolidateKind>();
    }
    solid8x8_keys.sort_by(|(ax, ay), (bx, by)| {
        if ay < by {
            return Ordering::Less;
        } else if ay > by {
            return Ordering::Greater;
        } else {
            return ax.cmp(bx);
        }
    });
    for key in solid8x8_keys {
        if !solid8x8_entries.contains_key(&key) {
            // Assume we've already removed it and move on
            continue;
        }
        let mut max_x_size = 1;
        while solid8x8_entries.contains_key(&(key.0 + max_x_size, key.1)) {
            max_x_size += 1;
        }
        fn can_support_y(
            key: (i32, i32),
            y: i32,
            max_x_size: i32,
            solid8x8_entries: &HashMap<(i32, i32), ConsolidationEntry>,
        ) -> bool {
            for xdiff in 0..max_x_size {
                if !solid8x8_entries.contains_key(&(key.0 + xdiff, key.1 + y)) {
                    return false;
                }
            }
            return true;
        }
        let mut max_y_size = 1;
        while can_support_y(key, max_y_size, max_x_size, &solid8x8_entries) {
            max_y_size += 1;
        }

        if max_x_size > 1 || max_y_size > 1 {
            let main_entry = solid8x8_entries.get(&key).unwrap().clone();
            let mut entries = vec![];
            for xdiff in 0..max_x_size {
                for ydiff in 0..max_y_size {
                    entries.push(
                        solid8x8_entries
                            .remove(&(key.0 + xdiff, key.1 + ydiff))
                            .unwrap(),
                    );
                }
            }
            for entry in entries {
                commands.entity(entry.eid).remove::<StaticTxCtrl>();
                // commands.entity(entry.comp_eid).despawn_recursive();
            }
            commands.entity(main_entry.eid).insert(StaticTx::single(
                StaticTxKind::Solid,
                HBox::new(max_x_size as u32 * 8, max_y_size as u32 * 8)
                    .translated((max_x_size - 1) as f32 * 4.0, (max_y_size - 1) as f32 * 4.0),
            ));
        }
    }
}

pub(super) fn register_my_ldtk_consolidate(app: &mut App) {
    app.observe(consolidate_int_cells);
}
