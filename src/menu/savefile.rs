use crate::prelude::*;

use super::menu_common::*;

#[derive(Resource)]
struct SavefileInput {
    selected: SavefileKind,
    block_move: Option<(f32, f32)>,
}

#[derive(Bundle)]
struct SavefileButtonBundle {
    name: Name,
    kind: SavefileKind,
    anim: AnimMan<SavefileButtonAnim>,
    spatial: SpatialBundle,
    temp: MenuTemp,
}
impl SavefileButtonBundle {
    fn new(kind: SavefileKind, y: f32) -> Self {
        Self {
            name: Name::new(format!("savefile_button_{:?}", kind)),
            kind,
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
}

fn watch_input(
    butts: Res<ButtInput>,
    dir: Res<DirInput>,
    mut commands: Commands,
    time: Res<Time>,
    mut input: ResMut<SavefileInput>,
    mut buttons: Query<(&SavefileKind, &mut AnimMan<SavefileButtonAnim>)>,
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
        if *kind == input.selected {
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
        commands.remove_resource::<Savefile>();
        commands.insert_resource(Savefile::change_me(input.selected));
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
        watch_input
            .after(InputSet)
            .run_if(in_state(MenuStateKind::Savefile))
            .run_if(in_state(TransitionActiveState::Inactive)),
    );
    app.add_systems(OnExit(MenuStateKind::Savefile), on_exit);
}
