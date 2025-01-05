use crate::prelude::*;

use super::menu_common::*;

fn on_enter(mut commands: Commands, ass: Res<AssetServer>, root: Res<MenuRoot>) {
    commands.spawn(MenuImage::new("menu/title.png"));
    commands.trigger(EndTransition::center());
    commands.trigger(SetupCanyonBg);

    // TODO: Make some helper component/function for this man (BSN WHEN!!!)
    let font_hand = ass.load("fonts/KodeMono/KodeMono-Bold.ttf");
    commands
        .spawn((
            Name::new("version"),
            Text2d::new(env!("CARGO_PKG_VERSION")),
            TextFont::from_font(font_hand).with_font_size(36.0),
            TextLayout::new_with_justify(JustifyText::Center),
            Anchor::TopLeft,
            Transform::from_translation(
                (Vec2::new(-WINDOW_VEC.x, WINDOW_VEC.y) / 2.0 + Vec2::new(4.0, 4.0))
                    .extend(ZIX_SPEEDRUN_TIMER),
            ),
            Visibility::Inherited,
            TextLayer::to_render_layers(),
            MenuTemp,
        ))
        .set_parent(root.eid());
}

fn watch_input(butts: Res<ButtInput>, mut commands: Commands) {
    if butts.pressed(ButtKind::Enter) || butts.pressed(ButtKind::A) {
        commands.trigger(StartTransition::to(MenuState::Savefile.to_meta_state()));
    }
}

fn on_exit(mut commands: Commands) {
    commands.trigger(CleanupMenuTemp);
}

pub(super) fn register_title(app: &mut App) {
    app.add_systems(OnEnter(MenuStateKind::Title), on_enter);
    app.add_systems(
        Update,
        watch_input
            .after(InputSet)
            .run_if(in_state(MenuStateKind::Title))
            .run_if(in_state(TransitionActiveState::Inactive)),
    );
    app.add_systems(OnExit(MenuStateKind::Title), on_exit);
}
