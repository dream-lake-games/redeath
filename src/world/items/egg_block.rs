use crate::prelude::*;

use super::egg::{egg_ghost_juice, EggGhostState};

#[derive(Bundle)]
struct EggBlockBundle {
    name: Name,
    pos: Pos,
    spatial: SpatialBundle,
    static_tx: StaticTx,
    anim: AnimMan<EggBlockAnim>,
}
impl MyLdtkEntity for EggBlockBundle {
    type Root = PlatformRoot;
    fn from_ldtk(pos: Pos, _: &HashMap<String, FieldValue>, _: String) -> Self {
        Self {
            name: Name::new("egg_block"),
            pos,
            spatial: pos.to_spatial(ZIX_ITEMS - 1.1),
            static_tx: StaticTx::single(StaticTxKind::Solid, HBox::new(16, 16)),
            anim: default(),
        }
    }
}

fn maybe_destroy_egg_blocks(
    eggs: Query<(&AnimMan<EggAnim>, &SpawnedLid)>,
    ghosts: Query<(&EggGhostState, &SpawnedLid)>,
    mut commands: Commands,
    mut blocks: Query<(Entity, &mut AnimMan<EggBlockAnim>, &SpawnedLid), With<StaticTxCtrl>>,
) {
    let mut slids_with_unbroken_eggs = HashSet::new();
    for (anim, slid) in &eggs {
        if !matches!(anim.get_state(), EggAnim::None | EggAnim::Break) {
            slids_with_unbroken_eggs.insert(slid.iid.clone());
        }
    }
    let mut slids_with_uncollected_ghosts = HashSet::new();
    for (state, slid) in &ghosts {
        if !matches!(state, EggGhostState::Collected) {
            slids_with_uncollected_ghosts.insert(slid.iid.clone());
        }
    }
    let mut popping = HashSet::new();
    for (eid, mut anim, slid) in &mut blocks {
        if !slids_with_unbroken_eggs.contains(&slid.iid)
            && !slids_with_uncollected_ghosts.contains(&slid.iid)
        {
            commands.entity(eid).remove::<StaticTxCtrl>();
            anim.set_state(EggBlockAnim::Pop);
            popping.insert(slid.iid.clone());
        }
    }
    for iid in popping {
        commands.trigger(EggBlocksPop { iid });
    }
}

pub(super) fn reset_egg_blocks(
    iid: &str,
    blocks: &mut Query<(
        Entity,
        &mut AnimMan<EggBlockAnim>,
        &SpawnedLid,
        Option<&StaticTxCtrl>,
    )>,
    commands: &mut Commands,
) {
    for (entity, mut anim, slid, stx_ctrl) in blocks {
        if slid.iid == iid {
            anim.set_state(EggBlockAnim::Solid);
            if stx_ctrl.is_none() {
                commands
                    .entity(entity)
                    .insert(StaticTx::single(StaticTxKind::Solid, HBox::new(16, 16)));
            }
        }
    }
}

#[derive(Event)]
pub(super) struct EggBlocksPop {
    pub(super) iid: String,
}

pub(super) fn register_egg_block(app: &mut App) {
    app.add_plugins(MyLdtkEntityPlugin::<EggBlockBundle>::new(
        "Entities", "EggBlock",
    ));

    app.add_systems(
        Update,
        maybe_destroy_egg_blocks
            .after(PhysicsSet)
            .after(egg_ghost_juice)
            .run_if(in_state(MetaStateKind::World)),
    );
}
