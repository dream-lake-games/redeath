use crate::prelude::*;

pub fn format_time(time_us: u128) -> String {
    let total_seconds = time_us / 1_000_000;
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;
    let milliseconds = (time_us % 1_000_000) / 1000;

    if hours > 0 {
        format!(
            "{:02}:{:02}:{:02}:{:03}",
            hours, minutes, seconds, milliseconds
        )
    } else {
        format!("{:02}:{:02}:{:03}", minutes, seconds, milliseconds)
    }
}

fn update_time(
    mut all_savefiles: ResMut<AllSavefiles>,
    current_savefile_kind: Res<CurrentSavefileKind>,
    time: Res<Time>,
) {
    let kind = current_savefile_kind.0.clone();
    let mut current = all_savefiles.map[&kind].clone();
    current.speedrun_timer_us += time.delta().as_micros();
    all_savefiles.map.insert(kind, current);
}

#[derive(Component, Default)]
struct SpeedrunTimer;

#[derive(Resource, Default)]
struct SpeedrunTimerVisible(bool);

#[derive(Bundle)]
struct SpeedrunTimerBundle {
    name: Name,
    marker: SpeedrunTimer,
    // pomegranate
    // text: Text2dBundle,
    render_layers: RenderLayers,
}
impl SpeedrunTimerBundle {
    fn new(time_us: u128, font_hand: Handle<Font>) -> Self {
        Self {
            name: Name::new("speedrun_time"),
            marker: SpeedrunTimer,

            // text: Text2dBundle {
            //     text: Text::from_section(
            //         format_time(time_us),
            //         TextStyle {
            //             font_size: 36.0,
            //             font: font_hand,
            //             ..default()
            //         },
            //     )
            //     .with_justify(JustifyText::Center),
            //     transform: Transform::from_translation(
            //         (-WINDOW_VEC / 2.0 + Vec2::new(4.0, 4.0)).extend(ZIX_SPEEDRUN_TIMER),
            //     ),
            //     text_anchor: bevy::sprite::Anchor::BottomLeft,
            //     ..default()
            // },
            render_layers: TextLayer::to_render_layers(),
        }
    }
}

fn update_current_time(
    mut commands: Commands,
    mut text_q: Query<&mut Text, With<SpeedrunTimer>>,
    all_savefiles: Res<AllSavefiles>,
    current_savefile_kind: Res<CurrentSavefileKind>,
    ass: Res<AssetServer>,
) {
    let font_hand = ass.load("fonts/KodeMono/KodeMono-Bold.ttf");
    let time_us = all_savefiles.map[&current_savefile_kind.into_inner().0].speedrun_timer_us;
    if text_q.is_empty() {
        commands.spawn(SpeedrunTimerBundle::new(time_us, font_hand));
    } else {
        let mut text = text_q.single_mut();
        // pomegranate
        // text.sections[0].value = format_time(time_us);
    }
}

fn show_current_time(
    mut visible_res: ResMut<SpeedrunTimerVisible>,
    meta_state_kind: Res<State<MetaStateKind>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut text_q: Query<(Entity, &mut Visibility), With<SpeedrunTimer>>,
    mut commands: Commands,
) {
    if meta_state_kind.get().clone() != MetaStateKind::World {
        for (eid, _) in &text_q {
            commands.entity(eid).despawn_recursive();
        }
        return;
    }
    if keyboard.just_pressed(KeyCode::ShiftLeft) {
        visible_res.0 = !visible_res.0;
    }
    for (_, mut viz) in &mut text_q {
        *viz = if visible_res.0 {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}

pub(super) fn register_speedrun_timer(app: &mut App) {
    app.insert_resource(SpeedrunTimerVisible::default());

    app.add_systems(
        Update,
        (update_time, update_current_time)
            .chain()
            .run_if(in_state(MetaStateKind::World))
            .run_if(in_state(PauseState::Unpaused))
            .run_if(in_state(ConvoMetaState::None))
            .run_if(in_state(CutsceneState::None)),
    );
    app.add_systems(Update, show_current_time.after(update_current_time));
}
