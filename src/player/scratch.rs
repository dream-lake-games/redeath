use crate::prelude::*;

#[derive(Bundle)]
pub(super) struct ScratchPlayerBundle {
    name: Name,
    player: Player,
    pos: Pos,
    dyno: Dyno,
    static_rx: StaticRx,
    sprite: SpriteBundle,
    render_layers: RenderLayers,
}
impl ScratchPlayerBundle {
    pub(super) fn new(pos: Pos) -> Self {
        let size = Vec2::new(6.0, 12.0);
        Self {
            name: Name::new("scratch_player"),
            player: Player,
            pos,
            dyno: default(),
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
        }
    }
}

pub(super) fn register_scratch(app: &mut App) {}
