use bevy::{
    asset::RenderAssetUsages,
    render::{
        mesh::{Indices, PrimitiveTopology},
        primitives::Aabb,
    },
};

use crate::prelude::*;

#[derive(Resource, Default)]
struct BlackMatRes(Option<Handle<ColorMaterial>>);

#[derive(Component)]
struct TemporaryLightMesh;

struct SolidLine {
    a: Vec2,
    b: Vec2,
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

fn hbox_to_blocked_mesh(source: Vec2, hbox: &HBox) -> (Mesh, Aabb) {
    let get_blocked = |p: Vec2| -> Vec2 { p + (p - source).normalize_or_zero() * MAX_LIGHT_RADIUS };

    let mut points = Vec::<Vec2>::new();
    let mut tris = Vec::<u32>::new();

    for line in hbox_to_solid_lines(hbox) {
        let first_ix = points.len() as u32;
        tris.extend([first_ix, first_ix + 1, first_ix + 2]);
        tris.extend([first_ix + 2, first_ix + 3, first_ix]);
        points.extend([line.a, get_blocked(line.a), get_blocked(line.b), line.b]);
    }

    let min_x = points
        .iter()
        .map(|p| p.x)
        .min_by(|a, b| a.total_cmp(b))
        .unwrap();
    let min_y = points
        .iter()
        .map(|p| p.y)
        .min_by(|a, b| a.total_cmp(b))
        .unwrap();
    let max_x = points
        .iter()
        .map(|p| p.x)
        .max_by(|a, b| a.total_cmp(b))
        .unwrap();
    let max_y = points
        .iter()
        .map(|p| p.y)
        .max_by(|a, b| a.total_cmp(b))
        .unwrap();
    let get_frac = |x: f32, min: f32, max: f32| (x - min) / (max - min);

    let mut inserted_positions = vec![];
    let mut inserted_uvs = vec![];
    let mut inserted_normals = vec![];

    for point in points.into_iter() {
        inserted_positions.push([point.x, point.y, 0.0]);
        inserted_uvs.push([
            get_frac(point.x, min_x, max_x),
            get_frac(point.y, min_y, max_y),
        ]);
        inserted_normals.push([0.0, 0.0, 1.0]);
    }

    (
        Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, inserted_positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, inserted_uvs)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, inserted_normals)
        .with_inserted_indices(Indices::U32(tris)),
        Aabb::enclosing([Vec3::new(min_x, min_y, 0.0), Vec3::new(max_x, max_y, 0.0)]).unwrap(),
    )
}

fn block_lights(
    mut black_mat: ResMut<BlackMatRes>,
    mut mats: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    light_root: Res<LightRoot>,
    old: Query<(Entity, &Mesh2d), With<TemporaryLightMesh>>,
    pos_q: Query<&Pos>,
    sources: Query<(Entity, &LightClaimed)>,
    blocker_ctrls: Query<&StaticTxCtrl, Without<Offscreen>>,
    blocker_comps: Query<&StaticTxComp>,
) {
    if black_mat.0.is_none() {
        black_mat.0 = Some(mats.add(Color::BLACK));
    }
    let black_mat = black_mat.0.clone().unwrap();

    let mut old_iter = old.iter();
    let mut make_or_reuse_mesh = |rl: RenderLayers, mesh: Mesh, aabb: Aabb| {
        if let Some((eid, emesh)) = old_iter.next() {
            let Some(mr) = meshes.get_mut(emesh.id()) else {
                return;
            };
            *mr = mesh;
            commands.entity(eid).insert((aabb, rl));
        } else {
            commands
                .spawn((
                    Name::new("temporary_mesh"),
                    Mesh2d(meshes.add(mesh).into()),
                    MeshMaterial2d(black_mat.clone()),
                    Transform::from_translation(Vec3::Z * 100.0),
                    Visibility::Inherited,
                    rl,
                    TemporaryLightMesh,
                ))
                .set_parent(light_root.eid());
        }
    };

    for (source_eid, light) in &sources {
        let source_pos = pos_q.get(source_eid).unwrap().as_vec2();
        for stx_ctrl in &blocker_ctrls {
            for comp_eid in &stx_ctrl.comps {
                let Ok(stx_comp) = blocker_comps.get(*comp_eid) else {
                    continue;
                };
                let blocker_pos = pos_q.get(stx_comp.ctrl).unwrap();
                let blocker_hbox = stx_comp.hbox.translated(blocker_pos.x, blocker_pos.y);
                if blocker_hbox.manhattan_distance_to_point(source_pos) > light.radius / 2.0 {
                    continue;
                }
                if blocker_hbox.manhattan_distance_to_point(source_pos) <= 0.001 {
                    // If a source is inside a box we want to ignore that box, useful for passup
                    continue;
                }
                if matches!(stx_comp.kind, StaticTxKind::SolidFragile) {
                    // For now we can see through solid fragile things...
                    continue;
                }
                let hbox = stx_comp.hbox.translated(blocker_pos.x, blocker_pos.y);
                let (mesh, aabb) = hbox_to_blocked_mesh(source_pos, &hbox);
                make_or_reuse_mesh(light.to_render_layers(), mesh, aabb);
            }
        }
    }

    drop(make_or_reuse_mesh);
    while let Some((eid, _)) = old_iter.next() {
        commands.entity(eid).despawn_recursive();
    }
}

pub(super) fn register_light_interaction(app: &mut App) {
    app.insert_resource(BlackMatRes::default());
    app.add_systems(
        Update,
        block_lights
            .after(PhysicsSet)
            .run_if(input_toggle_active(true, KeyCode::KeyL)),
    );
}
