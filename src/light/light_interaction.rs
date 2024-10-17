use bevy::sprite::MaterialMesh2dBundle;

use crate::prelude::*;

#[derive(Resource, Default)]
struct BlackMatRes(Option<Handle<ColorMaterial>>);

#[derive(Component)]
struct TemporaryLightMesh;

struct SolidLine {
    a: Vec2,
    b: Vec2,
}

struct BlockedQuad {
    a: Vec2,
    b: Vec2,
    c: Vec2,
    d: Vec2,
}
impl BlockedQuad {
    fn to_triangles(self) -> [Triangle2d; 2] {
        [
            Triangle2d::new(self.a, self.b, self.c),
            Triangle2d::new(self.c, self.d, self.a),
        ]
    }
}

fn hbox_to_solid_lines(hbox: &HBox) -> [SolidLine; 4] {
    let offset = hbox.get_offset();
    let size = hbox.get_size().as_vec2();
    let left = Vec2::X * size.x / 2.0;
    let up = Vec2::Y * size.y / 2.0;
    [
        SolidLine {
            a: offset - left - up,
            b: offset - left + up,
        },
        SolidLine {
            a: offset - left + up,
            b: offset + left + up,
        },
        SolidLine {
            a: offset + left + up,
            b: offset + left - up,
        },
        SolidLine {
            a: offset + left - up,
            b: offset - left - up,
        },
    ]
}

fn get_blocked_quad(source: Vec2, solid_line: SolidLine) -> BlockedQuad {
    let get_blocked = |p: Vec2| -> Vec2 { p + (p - source).normalize_or_zero() * MAX_LIGHT_RADIUS };
    let blocked_a = get_blocked(solid_line.a);
    let blocked_b = get_blocked(solid_line.b);
    BlockedQuad {
        a: solid_line.a,
        b: solid_line.b,
        c: blocked_b,
        d: blocked_a,
    }
}

fn hbox_to_blocked_quads(source: Vec2, hbox: &HBox) -> [BlockedQuad; 4] {
    let [l1, l2, l3, l4] = hbox_to_solid_lines(hbox);
    [
        get_blocked_quad(source, l1),
        get_blocked_quad(source, l2),
        get_blocked_quad(source, l3),
        get_blocked_quad(source, l4),
    ]
}

fn block_lights(
    mut black_mat: ResMut<BlackMatRes>,
    mut mats: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    light_root: Res<LightRoot>,
    old: Query<Entity, With<TemporaryLightMesh>>,
    pos_q: Query<&Pos>,
    sources: Query<(Entity, &LightClaimed)>,
    blockers: Query<&StaticTxComp>,
) {
    // Delete the old meshes
    for eid in &old {
        commands.entity(eid).despawn_recursive();
    }
    // Make sure we have the black mat
    if black_mat.0.is_none() {
        black_mat.0 = Some(mats.add(Color::BLACK));
    }
    let black_mat = black_mat.0.clone().unwrap();
    for (source_eid, light) in &sources {
        let source_pos = pos_q.get(source_eid).unwrap().as_vec2();
        for stx_comp in &blockers {
            let blocker_pos = pos_q.get(stx_comp.ctrl).unwrap();
            if source_pos.distance(blocker_pos.as_vec2()) > 64.0 {
                continue;
            }
            let hbox = stx_comp.hbox.translated(blocker_pos.x, blocker_pos.y);
            let blocked_quads = hbox_to_blocked_quads(source_pos, &hbox);
            for quad in blocked_quads {
                for triangle in quad.to_triangles() {
                    commands
                        .spawn((
                            Name::new("temporary_mesh"),
                            MaterialMesh2dBundle {
                                mesh: meshes.add(triangle).into(),
                                material: black_mat.clone(),
                                transform: Transform::from_translation(Vec3::Z * 100.0),
                                ..default()
                            },
                            light.to_render_layers(),
                            TemporaryLightMesh,
                        ))
                        .set_parent(light_root.eid());
                }
            }
        }
    }
}

pub(super) fn register_light_interaction(app: &mut App) {
    app.insert_resource(BlackMatRes::default());
    app.add_systems(BulletUpdate, block_lights.after(PhysicsSet));
}
