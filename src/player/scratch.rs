use crate::prelude::*;

#[derive(Bundle)]
pub(super) struct ScratchPlayerBundle {
    name: Name,
    player: Player,
    pos: Pos,
    dyno: Dyno,
    gravity: Gravity,
    static_rx: StaticRx,
    sprite: SpriteBundle,
    render_layers: RenderLayers,
    light: Light,
}
impl ScratchPlayerBundle {
    pub(super) fn new(pos: Pos) -> Self {
        let size = Vec2::new(9.0, 15.0);
        Self {
            name: Name::new("scratch_player"),
            player: Player,
            pos,
            dyno: default(),
            gravity: default(),
            static_rx: StaticRx::single(
                StaticRxKind::Default,
                HBox::new(size.x as u32, size.y as u32),
            ),
            sprite: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(size),
                    ..default()
                },
                transform: pos.to_spatial(ZIX_PLAYER).transform,
                ..default()
            },
            render_layers: MainLayer.into(),
            light: Light::new(LightAnim::Static128),
        }
    }
}

fn simple_movement(
    mut player_q: Query<&mut Dyno, With<Player>>,
    dir_input: Res<DirInput>,
    butt_input: Res<ButtInput>,
) {
    let mut dyno = player_q.single_mut();
    dyno.vel.x = dir_input.x * 100.0;
    if butt_input.just_pressed(ButtKind::A) {
        dyno.vel.y = 200.0;
    }
}

pub(super) fn register_scratch(app: &mut App) {
    app.add_systems(
        Update,
        simple_movement
            .run_if(in_state(PlayerMetaState::Playing))
            .after(InputSet)
            .after(PhysicsSet)
            .in_set(PlayerSet),
    );
}
