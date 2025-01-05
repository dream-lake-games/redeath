use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

use crate::prelude::*;

#[derive(Component)]
struct InGameFpsTimer;

fn startup(mut commands: Commands, debug_root: Res<DebugRoot>, ass: Res<AssetServer>) {
    let font_hand = ass.load("fonts/KodeMono/KodeMono-Bold.ttf");
    commands
        .spawn((
            Name::new("in_game_fps_time"),
            InGameFpsTimer,
            Text2d::new(String::new()),
            TextFont::from_font(font_hand).with_font_size(32.0),
            TextLayout::new_with_justify(JustifyText::Center),
            Anchor::TopRight,
            Transform::from_translation(
                (WINDOW_VEC / 2.0 - Vec2::new(4.0, 4.0)).extend(ZIX_SPEEDRUN_TIMER),
            ),
            Visibility::Inherited,
            TextLayer::to_render_layers(),
        ))
        .set_parent(debug_root.eid());
}

#[derive(Resource)]
struct InGameFpsVisible(bool);
fn update_in_game_fps(
    mut text_q: Query<(&mut Text2d, &mut Visibility), With<InGameFpsTimer>>,
    diagnostics: Res<DiagnosticsStore>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut in_game_fps_visible: ResMut<InGameFpsVisible>,
) {
    let Ok((mut text, mut viz)) = text_q.get_single_mut() else {
        return;
    };
    if let Some(value) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
    {
        text.0 = format!("FPS: {value:>4.0}");
        *viz = if in_game_fps_visible.0 {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
    if keyboard.just_pressed(KeyCode::KeyF) {
        in_game_fps_visible.0 = !in_game_fps_visible.0;
    }
}

pub(super) fn register_in_game_fps(app: &mut App) {
    app.insert_resource(InGameFpsVisible(false));
    app.add_systems(Startup, startup.after(RootInit));
    app.add_systems(Update, update_in_game_fps);
}
