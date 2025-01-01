use crate::{misc::format_speedrun_time, prelude::*};

use super::menu_common::*;

#[derive(Resource)]
struct SavefileInput {
    selected: SavefileKind,
    block_move: Option<(f32, f32)>,
}

struct WrappedKind(SavefileKind);
impl Component for WrappedKind {
    const STORAGE_TYPE: StorageType = StorageType::SparseSet;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            let myself = world.get::<Self>(eid).unwrap();
            let my_name = myself.0.to_string();
            let font_hand = world
                .resource::<AssetServer>()
                .load("fonts/KodeMono/KodeMono-Bold.ttf");
            let all_savefiles = world.resource::<AllSavefiles>();
            let this_savefile = all_savefiles.map[&myself.0].clone();

            let chapters_complete = this_savefile
                .world_savefiles
                .values()
                .filter(|world_data| world_data.ever_finished)
                .count();
            let total_chapters = 1;

            let coins_collected = this_savefile
                .world_savefiles
                .values()
                .map(|chapter| chapter.collected_coins.len())
                .reduce(|a, b| a + b)
                .unwrap();
            let total_coins = 26;

            let speedrun_time = format_speedrun_time(this_savefile.speedrun_timer_us);
            let num_deaths = this_savefile.num_deaths;

            macro_rules! spawn_text {
                ($text:expr, $font_size:expr, $x:expr, $y:expr$(,)?) => {
                    world
                        .commands()
                        .spawn((
                            Text2dBundle {
                                text: Text::from_section(
                                    $text,
                                    TextStyle {
                                        font_size: $font_size,
                                        font: font_hand.clone(),
                                        ..default()
                                    },
                                ),
                                transform: Transform::from_translation(Vec3::new($x, $y, 1.0)),
                                ..default()
                            },
                            MenuLayer::to_render_layers(),
                        ))
                        .set_parent(eid);
                };
            }

            spawn_text!(my_name, 18.0, -64.0, 10.0);
            spawn_text!(speedrun_time, 12.0, -58.0, -10.0);
            spawn_text!(
                format!("Chapters: {chapters_complete}/{total_chapters}",),
                18.0,
                24.0,
                10.0,
            );
            spawn_text!(
                format!("C:{coins_collected}/{total_coins}, D:{num_deaths}",),
                18.0,
                24.0,
                -10.0,
            );
        });
    }
}

#[derive(Bundle)]
struct SavefileButtonBundle {
    name: Name,
    kind: WrappedKind,
    anim: AnimMan<SavefileButtonAnim>,
    spatial: SpatialBundle,
    temp: MenuTemp,
}
impl SavefileButtonBundle {
    fn new(kind: SavefileKind, y: f32) -> Self {
        Self {
            name: Name::new(format!("savefile_button_{:?}", kind)),
            kind: WrappedKind(kind),
            anim: default(),
            spatial: Pos::new(0.0, y).to_spatial(1.0),
            temp: MenuTemp,
        }
    }
}

fn on_enter(mut commands: Commands) {
    commands.spawn(MenuImage::new("menu/savefile.png"));
    commands.trigger(EndTransition::center());

    commands.insert_resource(SavefileInput {
        selected: SavefileKind::A,
        block_move: None,
    });
    commands.spawn(SavefileButtonBundle::new(SavefileKind::A, 32.0));
    commands.spawn(SavefileButtonBundle::new(SavefileKind::B, -16.0));
    commands.spawn(SavefileButtonBundle::new(SavefileKind::C, -64.0));

    commands.insert_resource(HoldDeletionTime(0.0));

    commands.trigger(SetupCanyonBg);
}

fn watch_input(
    butts: Res<ButtInput>,
    dir: Res<DirInput>,
    mut commands: Commands,
    time: Res<Time>,
    mut input: ResMut<SavefileInput>,
    mut buttons: Query<(&WrappedKind, &mut AnimMan<SavefileButtonAnim>)>,
    mut current_savefile_kind: ResMut<CurrentSavefileKind>,
) {
    // Go back if we need
    if butts.pressed(ButtKind::Escape) || butts.pressed(ButtKind::B) {
        commands.trigger(StartTransition::to(MenuState::Title.to_meta_state()));
        return;
    }
    // Maybe turn off block move
    if let Some((signum, time_left)) = input.block_move.as_mut() {
        if (dir.length_squared() < 0.5) || (dir.y.signum() != *signum) || (*time_left <= 0.0) {
            input.block_move = None;
        } else {
            *time_left -= time.delta_seconds();
        }
    }
    // Maybe switch savefiles
    if input.block_move.is_none() && dir.y.abs() != 0.0 {
        let new = if dir.y < 0.0 {
            input.selected.next()
        } else {
            input.selected.prev()
        };
        if let Some(new_kind) = new {
            input.selected = new_kind;
            input.block_move = Some((dir.y.signum(), 0.28));
        }
    }
    // Update the buttons
    for (kind, mut anim) in &mut buttons {
        if kind.0 == input.selected {
            if anim.get_state() != SavefileButtonAnim::Active {
                anim.set_state(SavefileButtonAnim::Activate);
            }
        } else {
            anim.set_state(SavefileButtonAnim::Idle);
        }
    }
    // Go forward if we can
    if butts.pressed(ButtKind::Enter) || butts.pressed(ButtKind::A) {
        commands.trigger(StartTransition::to(
            MenuState::OverworldLoading.to_meta_state(),
        ));
        current_savefile_kind.0 = input.selected;
        commands.trigger(SavefileGetRecalculate);
    }
}

#[derive(Resource)]
struct HoldDeletionTime(f32);
fn maybe_delete_savefile(
    mut hold_deletion_time: ResMut<HoldDeletionTime>,
    input: Res<SavefileInput>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut current_savefile_kind: ResMut<CurrentSavefileKind>,
) {
    if hold_deletion_time.0 > 1.0 {
        commands.trigger(SavefileResetEvent {
            kind: input.selected,
        });
        commands.trigger(StartTransition::to(
            MenuState::OverworldLoading.to_meta_state(),
        ));
        current_savefile_kind.0 = input.selected;
        commands.trigger(SavefileGetRecalculate);
    } else {
        if keyboard.pressed(KeyCode::Backspace) {
            hold_deletion_time.0 += time.delta_seconds();
        } else {
            hold_deletion_time.0 = 0.0;
        }
    }
}

fn on_exit(mut commands: Commands) {
    commands.trigger(CleanupMenuTemp);
    commands.remove_resource::<SavefileInput>();
}

pub(super) fn register_savefile(app: &mut App) {
    app.add_systems(OnEnter(MenuStateKind::Savefile), on_enter);
    app.add_systems(
        Update,
        (watch_input, maybe_delete_savefile)
            .after(InputSet)
            .run_if(in_state(MenuStateKind::Savefile))
            .run_if(in_state(TransitionActiveState::Inactive)),
    );
    app.add_systems(OnExit(MenuStateKind::Savefile), on_exit);
}
