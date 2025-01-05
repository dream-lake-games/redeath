use crate::prelude::*;

fn cleanup(commands: &mut Commands, root: Entity) {
    commands.entity(root).despawn_descendants();
}

#[derive(Event)]
pub struct SetupCanyonBg;
fn handle_setup_canyon_bg(
    _trigger: Trigger<SetupCanyonBg>,
    mut commands: Commands,
    root: Res<BgFgRoot>,
    mut storm: ResMut<StormManager>,
) {
    cleanup(&mut commands, root.eid());

    commands
        .spawn(MediumRainBundle::new())
        .set_parent(root.eid());
    storm.show_rain();

    commands.spawn(BlackScreenImage);
    commands.spawn(BgClouds::new(
        "environment/bg/clouds_far.png",
        480,
        184,
        0.0,
        0.01,
        1.0,
    ));
    commands
        .spawn((
            Name::new("mountains_far"),
            ParallaxScreenImage::new_bg("environment/bg/mountains_far.png", 720, 184, 5.0)
                .with_parallax_x(0.03),
        ))
        .set_parent(root.eid());
    commands
        .spawn(BgClouds::new(
            "environment/bg/clouds_close.png",
            480,
            184,
            10.0,
            0.02,
            3.0,
        ))
        .set_parent(root.eid());
    commands
        .spawn((
            Name::new("mountains_close"),
            ParallaxScreenImage::new_bg("environment/bg/mountains_close.png", 720, 184, 15.0)
                .with_parallax_x(0.12),
        ))
        .set_parent(root.eid());
    commands.trigger(SpawnStarsEvent {
        num3s: 12,
        num5s: 24,
        num7s: 12,
    });
}

pub(super) fn register_bg_prebuilt(app: &mut App) {
    app.add_observer(handle_setup_canyon_bg);
}
