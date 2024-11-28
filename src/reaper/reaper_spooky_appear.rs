use crate::prelude::*;

#[derive(Component)]
struct ReaperSpookyPreAppear {
    will_convo: bool,
}

#[derive(Component)]
struct ReaperSpookyConvoing;

#[derive(Clone)]
struct ReaperSpookyAppearInner {
    time_till_disappear: f32,
}
impl Default for ReaperSpookyAppearInner {
    fn default() -> Self {
        Self {
            time_till_disappear: 1.0,
        }
    }
}
impl Component for ReaperSpookyAppearInner {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            let myself = world.get::<Self>(eid).expect("myself").clone();
            world.resource_mut::<WavyPaletteManager>().set(
                WavyPalette::new(BERRY_NEBULA)
                    .with_r(-0.02, 0.02, myself.time_till_disappear)
                    .with_g(-0.1, 0.01, myself.time_till_disappear / 4.0)
                    .with_b(-0.1, 0.01, myself.time_till_disappear / 5.0),
            );
            world
                .resource_mut::<BulletTime>()
                .set_temp(BulletTimeSpeed::Slow, 0.25);
            world
                .resource_mut::<CameraShake>()
                .shake(0.35, -1..=1, -1..=1);
        });
        hooks.on_remove(|mut world, _, _| {
            world.resource_mut::<WavyPaletteManager>().set(
                WavyPalette::new(BERRY_NEBULA)
                    .with_r(-0.06, 0.02, 2.0)
                    .with_g(-0.18, 0.02, 3.0)
                    .with_b(-0.18, 0.02, 5.0),
            );
        });
    }
}

#[derive(Bundle)]
pub struct ReaperSpookyAppear {
    name: Name,
    pos: Pos,
    spatial: SpatialBundle,
    trigger_tx: TriggerTx,
    preappear: ReaperSpookyPreAppear,
}
impl ReaperSpookyAppear {
    pub fn new(pos: Pos, trigger_offset: Vec2, trigger_height: f32, will_convo: bool) -> Self {
        let trigger_hbox = HBox::new(2, trigger_height.round() as u32)
            .translated(trigger_offset.x, trigger_offset.y);
        Self {
            name: Name::new("reaper_spooky_appear"),
            pos,
            spatial: pos.to_spatial(ZIX_PLAYER - 0.5),
            trigger_tx: TriggerTx::single(TriggerTxKind::Player, trigger_hbox),
            preappear: ReaperSpookyPreAppear { will_convo },
        }
    }
}
impl MyLdtkEntity for ReaperSpookyAppear {
    type Root = WorldMetaRoot;
    fn from_ldtk(pos: Pos, fields: &HashMap<String, FieldValue>, _iid: String) -> Self {
        let (Some(FieldValue::Float(Some(x))), Some(FieldValue::Float(Some(y)))) =
            (fields.get("TriggerXOffset"), fields.get("TriggerYOffset"))
        else {
            panic!("bad reaperspookyappear offsets");
        };
        let Some(FieldValue::Float(Some(height))) = fields.get("TriggerHeight") else {
            panic!("bad reaperspookyappear height");
        };
        let Some(FieldValue::Bool(will_convo)) = fields.get("Convo") else {
            panic!("bad reaperspookyappear convo");
        };
        Self::new(pos, Vec2::new(*x, *y), *height, *will_convo)
    }
}

fn maybe_start_appearing(
    mut commands: Commands,
    appears: Query<(Entity, &TriggerTxCtrl, &ReaperSpookyPreAppear)>,
    trigger_colls: Res<TriggerColls>,
    mut song_manager: ResMut<SongManager>,
    mut global_shift: ResMut<GlobalPaletteShift>,
) {
    for (eid, ttx_ctrl, preappear) in &appears {
        if trigger_colls
            .get_refs(&ttx_ctrl.coll_keys)
            .iter()
            .any(|coll| coll.rx_kind == TriggerRxKind::Player)
        {
            if let Some(mut comms) = commands.get_entity(eid) {
                comms.insert(AnimMan::<ReaperAnim>::new(ReaperAnim::Appear));
                comms.insert(Light::<LightStatic64Anim>::default());
                if preappear.will_convo {
                    comms.insert(ReaperSpookyConvoing);
                }
                comms.insert(ReaperSpookyAppearInner::default());
                comms.remove::<ReaperSpookyPreAppear>();
            }
            if song_manager.get_current() == Song::NoSong {
                song_manager.fade_to(Song::FightAmidstTheDestructionIntro);
            }
            commands.spawn((SoundEffect::PlayerThunder, SoundMult(1.75)));
            commands.spawn(Lightning);
            global_shift.add(0.1, 1);
            global_shift.add(0.2, 1);
            global_shift.add(0.3, 1);
        }
    }
}

fn update_reaper_spooky_appeared(
    mut reapers: Query<
        (&mut ReaperSpookyAppearInner, &mut AnimMan<ReaperAnim>),
        Without<ReaperSpookyConvoing>,
    >,
    bullet_time: Res<BulletTime>,
    mut camera_shake: ResMut<CameraShake>,
    mut commands: Commands,
    mut global_shift: ResMut<GlobalPaletteShift>,
) {
    for (mut inner, mut anim) in &mut reapers {
        inner.time_till_disappear -= bullet_time.delta_seconds();
        if inner.time_till_disappear <= 0.0 && anim.get_state() != ReaperAnim::Disappear {
            anim.set_state(ReaperAnim::Disappear);
            camera_shake.shake(0.2, -1..=1, -1..=1);
            commands.spawn((SoundEffect::PlayerThunder, SoundMult(1.75)));
            commands.spawn(Lightning);
            global_shift.add(0.1, 1);
            global_shift.add(0.2, 1);
            global_shift.add(0.3, 1);
        }
    }
}

fn remove_reaper_spooky_convoing(
    _trigger: Trigger<ConvoCanyonEnded>,
    mut commands: Commands,
    ents: Query<Entity, With<ReaperSpookyConvoing>>,
) {
    for eid in &ents {
        if let Some(mut comms) = commands.get_entity(eid) {
            comms.remove::<ReaperSpookyConvoing>();
        }
    }
}

pub(super) fn register_reaper_spooky_appear(app: &mut App) {
    app.add_plugins(MyLdtkEntityPlugin::<ReaperSpookyAppear>::new(
        "Entities",
        "ReaperSpookyAppear",
    ));

    app.observe(remove_reaper_spooky_convoing);

    app.add_systems(
        Update,
        (maybe_start_appearing, update_reaper_spooky_appeared)
            .run_if(in_state(MetaStateKind::World))
            .after(PhysicsSet),
    );
}
