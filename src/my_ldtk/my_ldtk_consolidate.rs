//! TODO: Maybe at some point I should not copy paste here and extract to common logic huh...
//!       Ehhh but maybe not

use std::cmp::Ordering;

use crate::prelude::*;

#[derive(Component, Debug, Clone, Reflect)]
pub enum MyLdtkConsolidateKind {
    /// For consolidating things which are a single solid 8x8 hitbox
    Solid8x8,
    /// For consolidating spike hitboxes
    Spike6x6,
}

#[derive(Clone)]
struct ConsolidationEntry {
    eid: Entity,
}

fn consolidate_solid_8x8(
    _trigger: Trigger<LevelChangeEvent>,
    consolidating: Query<(
        Entity,
        &MyLdtkConsolidateKind,
        &StaticTxCtrl,
        &Pos,
        &SpawnedLid,
    )>,
    stx_comps: Query<&StaticTxComp>,
    mut commands: Commands,
) {
    fn get_key(pos: &Pos, slid: String) -> (i32, i32, String) {
        (
            (pos.x / 8.0).floor() as i32,
            (pos.y / 8.0).floor() as i32,
            slid,
        )
    }

    let mut solid8x8_keys: Vec<(i32, i32, String)> =
        Vec::with_capacity(consolidating.iter().count());
    let mut solid8x8_entries: HashMap<(i32, i32, String), ConsolidationEntry> = default();
    for (eid, kind, ctrl, pos, slid) in &consolidating {
        let key = get_key(pos, slid.iid.clone());
        match kind {
            MyLdtkConsolidateKind::Solid8x8 => {
                debug_assert!(ctrl.comps.len() == 1);
                let comp = stx_comps.get(ctrl.comps[0]).unwrap();
                debug_assert!(comp.kind == StaticTxKind::Solid);
                debug_assert!(comp.hbox.get_offset() == Vec2::ZERO);
                debug_assert!(comp.hbox.get_size() == UVec2::new(8, 8));
                solid8x8_keys.push(key.clone());
                solid8x8_entries.insert(key.clone(), ConsolidationEntry { eid });
            }
            _ => continue,
        }
        commands.entity(eid).remove::<MyLdtkConsolidateKind>();
    }
    solid8x8_keys.sort_by(|(ax, ay, _), (bx, by, _)| {
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
        while solid8x8_entries.contains_key(&(key.0 + max_x_size, key.1, key.2.clone())) {
            max_x_size += 1;
        }
        fn can_support_y(
            key: (i32, i32, String),
            y: i32,
            max_x_size: i32,
            solid8x8_entries: &HashMap<(i32, i32, String), ConsolidationEntry>,
        ) -> bool {
            for xdiff in 0..max_x_size {
                if !solid8x8_entries.contains_key(&(key.0 + xdiff, key.1 + y, key.2.clone())) {
                    return false;
                }
            }
            return true;
        }
        let mut max_y_size = 1;
        while can_support_y(key.clone(), max_y_size, max_x_size, &solid8x8_entries) {
            max_y_size += 1;
        }

        if max_x_size > 1 || max_y_size > 1 {
            let main_entry = solid8x8_entries.get(&key).unwrap().clone();
            let mut entries = vec![];
            for xdiff in 0..max_x_size {
                for ydiff in 0..max_y_size {
                    entries.push(
                        solid8x8_entries
                            .remove(&(key.0 + xdiff, key.1 + ydiff, key.2.clone()))
                            .unwrap(),
                    );
                }
            }
            for entry in entries {
                commands.entity(entry.eid).remove::<StaticTxCtrl>();
            }
            commands.entity(main_entry.eid).insert(StaticTx::single(
                StaticTxKind::Solid,
                HBox::new(max_x_size as u32 * 8, max_y_size as u32 * 8)
                    .translated((max_x_size - 1) as f32 * 4.0, (max_y_size - 1) as f32 * 4.0),
            ));
        }
    }
}

fn consolidate_spike_6x6(
    _trigger: Trigger<LevelChangeEvent>,
    consolidating: Query<(
        Entity,
        &MyLdtkConsolidateKind,
        &TriggerTxCtrl,
        &Pos,
        &SpawnedLid,
    )>,
    ttx_comps: Query<&TriggerTxComp>,
    mut commands: Commands,
) {
    fn get_key(pos: &Pos, slid: String) -> (i32, i32, String) {
        (
            (pos.x / 8.0).floor() as i32,
            (pos.y / 8.0).floor() as i32,
            slid,
        )
    }

    let mut spike6x6_keys: Vec<(i32, i32, String)> =
        Vec::with_capacity(consolidating.iter().count());
    let mut spike6x6_entries: HashMap<(i32, i32, String), ConsolidationEntry> = default();
    for (eid, kind, ctrl, pos, slid) in &consolidating {
        let key = get_key(pos, slid.iid.clone());
        match kind {
            MyLdtkConsolidateKind::Spike6x6 => {
                debug_assert!(ctrl.comps.len() == 1);
                let comp = ttx_comps.get(ctrl.comps[0]).unwrap();
                debug_assert!(comp.kind == TriggerTxKind::Spikes);
                debug_assert!(comp.hbox.get_offset() == Vec2::ZERO);
                debug_assert!(comp.hbox.get_size() == UVec2::new(6, 6));
                spike6x6_keys.push(key.clone());
                spike6x6_entries.insert(key.clone(), ConsolidationEntry { eid });
            }
            _ => {
                continue;
            }
        }
        commands.entity(eid).remove::<MyLdtkConsolidateKind>();
    }
    spike6x6_keys.sort_by(|(ax, ay, _), (bx, by, _)| {
        if ay < by {
            return Ordering::Less;
        } else if ay > by {
            return Ordering::Greater;
        } else {
            return ax.cmp(bx);
        }
    });
    for key in spike6x6_keys {
        if !spike6x6_entries.contains_key(&key) {
            // Assume we've already removed it and move on
            continue;
        }
        let mut max_x_size = 1;
        while spike6x6_entries.contains_key(&(key.0 + max_x_size, key.1, key.2.clone())) {
            max_x_size += 1;
        }
        fn can_support_y(
            key: (i32, i32, String),
            y: i32,
            max_x_size: i32,
            solid8x8_entries: &HashMap<(i32, i32, String), ConsolidationEntry>,
        ) -> bool {
            for xdiff in 0..max_x_size {
                if !solid8x8_entries.contains_key(&(key.0 + xdiff, key.1 + y, key.2.clone())) {
                    return false;
                }
            }
            return true;
        }
        let mut max_y_size = 1;
        while can_support_y(key.clone(), max_y_size, max_x_size, &spike6x6_entries) {
            max_y_size += 1;
        }

        if max_x_size > 1 || max_y_size > 1 {
            let main_entry = spike6x6_entries.get(&key).unwrap().clone();
            let mut entries = vec![];
            for xdiff in 0..max_x_size {
                for ydiff in 0..max_y_size {
                    entries.push(
                        spike6x6_entries
                            .remove(&(key.0 + xdiff, key.1 + ydiff, key.2.clone()))
                            .unwrap(),
                    );
                }
            }
            for entry in entries {
                commands.entity(entry.eid).remove::<TriggerTxCtrl>();
            }
            commands.entity(main_entry.eid).insert(TriggerTx::single(
                TriggerTxKind::Spikes,
                HBox::new(max_x_size as u32 * 8 - 2, max_y_size as u32 * 8 - 2)
                    .translated((max_x_size - 1) as f32 * 4.0, (max_y_size - 1) as f32 * 4.0),
            ));
        }
    }
}

pub(super) fn register_my_ldtk_consolidate(app: &mut App) {
    app.add_observer(consolidate_solid_8x8);
    app.add_observer(consolidate_spike_6x6);
}
