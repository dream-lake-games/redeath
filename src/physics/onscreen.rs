use crate::prelude::*;

#[derive(Component)]
pub struct Onscreen;

#[derive(Component)]
pub struct Offscreen;

#[derive(Component)]
struct UpdateOnscreen;
#[derive(Component)]
struct UpdateOnscreenKind<const KIND: u32>;

/// This is the next kind to assign to something to keep it balanced
#[derive(Resource)]
struct NextUpdateOnscreenKind(u32);
const NUM_UPDATE_ONSCREEN_KINDS: u32 = 6;
impl NextUpdateOnscreenKind {
    fn next(&mut self) -> u32 {
        let val = self.0;
        self.0 += 1;
        self.0 = self.0 % NUM_UPDATE_ONSCREEN_KINDS;
        val
    }
}

/// This is the next kind to UPDATE to keep it balanced (this name sucks)
#[derive(Resource)]
struct NextUpdateOnscreenRun(u32);
impl NextUpdateOnscreenRun {
    fn next(&mut self) -> u32 {
        let val = self.0;
        self.0 += 1;
        self.0 = self.0 % NUM_UPDATE_ONSCREEN_KINDS;
        val
    }
}

/// Gives an `UpdateOnscreen` to anything with hitbox ctrl + pos that doesn't have one
fn bless_update_onscreen(
    mut next_kind: ResMut<NextUpdateOnscreenKind>,
    in_need: Query<
        Entity,
        (
            With<Pos>,
            Or<(
                With<StaticTxCtrl>,
                With<StaticRxCtrl>,
                With<TriggerTxCtrl>,
                With<TriggerRxCtrl>,
            )>,
            Without<UpdateOnscreen>,
        ),
    >,
    mut commands: Commands,
) {
    macro_rules! add_the_thing {
        ($eid:expr, $val:expr, $($kind:literal$(,)?)+) => {
            match $val {
                $(
                    $kind => {
                        if let Some(mut comms) = commands.get_entity($eid) {
                            comms.insert(UpdateOnscreenKind::<$kind>);
                        }
                    }
                )+
                _ => {
                    panic!("ruh roh can't add the thing");
                }
            }
        };
    }

    for eid in &in_need {
        add_the_thing!(eid, next_kind.next(), 0, 1, 2, 3, 4, 5);
    }
}

/// This is the logic that sets onscreen info for one "kind"
/// This will be called evbery frame, but only one "kind"
/// We try to divide the kinds up evenly
/// We do it like this because it's probably too much work to do for all kinds every frame
/// but we don't want this calculation to introduce lag spikes, so we do a little
/// bit of the work every frame
#[derive(Event)]
struct DoUpdateOnScreen<const KIND: u32>;
fn do_update_onscreen<const KIND: u32>(
    _trigger: Trigger<DoUpdateOnScreen<KIND>>,
    camera_pos: Query<&Pos, With<DynamicCamera>>,
    relevant: Query<
        (
            Entity,
            &Pos,
            Option<&StaticTxCtrl>,
            Option<&StaticRxCtrl>,
            Option<&TriggerTxCtrl>,
            Option<&TriggerRxCtrl>,
        ),
        With<UpdateOnscreenKind<KIND>>,
    >,
    stx_comps: Query<&StaticTxComp>,
    srx_comps: Query<&StaticRxComp>,
    ttx_comps: Query<&TriggerTxComp>,
    trx_comps: Query<&TriggerRxComp>,
    mut commands: Commands,
) {
    let Ok(cam_pos) = camera_pos.get_single() else {
        warn!(
            "Can't update on screen for {:?} because can't get single cam pos",
            KIND
        );
        return;
    };
    for (eid, pos, maybe_stx_ctrl, maybe_srx_ctrl, maybe_ttx_ctrl, maybe_trx_ctrl) in &relevant {
        let Some(mut comms) = commands.get_entity(eid) else {
            continue;
        };
        let mut hboxes = vec![];

        macro_rules! account_for_hitboxes {
            ($maybe:expr, $query:expr) => {
                $maybe
                    .map(|inner| inner.comps.clone())
                    .unwrap_or(vec![])
                    .into_iter()
                    .for_each(|comp_eid| {
                        if let Ok(comp) = $query.get(comp_eid) {
                            hboxes.push(comp.hbox.clone())
                        }
                    });
            };
        }

        account_for_hitboxes!(maybe_stx_ctrl, stx_comps);
        account_for_hitboxes!(maybe_srx_ctrl, srx_comps);
        account_for_hitboxes!(maybe_ttx_ctrl, ttx_comps);
        account_for_hitboxes!(maybe_trx_ctrl, trx_comps);

        let onscreen = hboxes.iter().any(|hbox| {
            hbox.translated(pos.x, pos.y)
                .manhattan_distance_to_point(cam_pos.as_vec2())
                < SCREEN_WIDTH_f32
        });

        if onscreen {
            comms.insert(Onscreen);
            comms.remove::<Offscreen>();
        } else {
            comms.insert(Offscreen);
            comms.remove::<Onscreen>();
        }
    }
}

fn do_update_onscreen_run(mut next_run: ResMut<NextUpdateOnscreenRun>, mut commands: Commands) {
    let val = next_run.next();
    macro_rules! trigger_specific_runs {
        ($val:expr, $($kind:literal$(,)?)+) => {
            match $val {
                $(
                    $kind => {
                        commands.trigger(DoUpdateOnScreen::<$kind>);
                    }
                )+
                _ => panic!("bad kind in do_update_onscreen_run"),
            }
        };
    }
    trigger_specific_runs!(val, 0, 1, 2, 3, 4, 5);
}

pub(super) fn register_onscreen(app: &mut App) {
    app.insert_resource(NextUpdateOnscreenKind(0));
    app.insert_resource(NextUpdateOnscreenRun(0));
    app.add_systems(
        PreUpdate,
        (bless_update_onscreen, do_update_onscreen_run).chain(),
    );

    macro_rules! observe_specific_runs {
        ($($kind:literal$(,)?)+) => {
            $(
                app.observe(do_update_onscreen::<$kind>);
            )+
        };
    }
    observe_specific_runs!(0, 1, 2, 3, 4, 5);
}
