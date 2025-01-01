use std::ops::SubAssign;

use bevy::ecs::query::QuerySingleError;

use crate::prelude::*;

#[derive(Bundle, Clone, Debug)]
pub struct ConvoBox {
    name: Name,
    root: ConvoBoxRoot,
    spatial: SpatialBundle,
    pub speaker: ConvoSpeaker,
    pub portrait: ConvoPortrait,
    pub sound: ConvoSound,
    pub text: ConvoText,
}
impl ConvoBox {
    pub fn new(speaker: ConvoSpeaker, portrait: &str, sound: &str, text: ConvoText) -> Self {
        Self {
            name: Name::new("box_root"),
            root: ConvoBoxRoot,
            spatial: SpatialBundle {
                visibility: Visibility::Hidden,
                transform: Transform::from_translation(Vec3::new(
                    0.0,
                    -56.0 * TextLayer::growth_factor() as f32,
                    0.0,
                )),
                ..default()
            },
            speaker,
            portrait: ConvoPortrait {
                key: portrait.to_string(),
            },
            sound: ConvoSound {
                key: sound.to_string(),
            },
            text,
        }
    }
}

#[derive(Clone, Debug, Reflect)]
struct ConvoBoxRoot;

#[derive(Component)]
struct ConvoBoxRootLoading {
    frames_left: u32,
}

impl Component for ConvoBoxRoot {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            // Set the right parent, insert the loading waiter, gift hidden spatial
            let convo_root_eid = world.resource::<ConvoRoot>().eid();
            let box_root_eid = world
                .commands()
                .entity(eid)
                .set_parent(convo_root_eid)
                .insert(ConvoBoxRootLoading { frames_left: 6 })
                .id();

            // Spawn the background
            let hand = world.resource::<AssetServer>().load("convo/bg.png");
            world
                .commands()
                .spawn((
                    Name::new("bg"),
                    SpriteBundle {
                        texture: hand,
                        transform: Transform {
                            translation: Vec3::new(0.0, 0.0, ZIX_CONVO_BG),
                            scale: (Vec2::ONE * TextLayer::growth_factor() as f32).extend(1.0),
                            ..default()
                        },
                        ..default()
                    },
                    TextLayer::to_render_layers(),
                ))
                .set_parent(box_root_eid);
        });
    }
}

#[derive(Component, Debug)]
pub struct ConvoManager {
    boxes: Vec<ConvoBox>,
}
impl ConvoManager {
    pub fn new<I: DoubleEndedIterator<Item = ConvoBox>>(boxes: I) -> Self {
        Self {
            boxes: boxes.into_iter().rev().collect(),
        }
    }
}

/// Finishes loading a single box, despawning stale ones
fn finish_loading_box(
    mut commands: Commands,
    mut loading: Query<
        (
            Entity,
            &mut ConvoBoxRootLoading,
            &mut Visibility,
            &ConvoSpeaker,
        ),
        With<ConvoBoxRoot>,
    >,
    stale: Query<Entity, (With<ConvoBoxRoot>, Without<ConvoBoxRootLoading>)>,
) {
    let (eid, mut loading, mut viz, speaker) = match loading.get_single_mut() {
        Ok(loading) => loading,
        Err(QuerySingleError::NoEntities(_)) => return,
        Err(QuerySingleError::MultipleEntities(_)) => {
            warn!("multiple convo boxes loading...");
            return;
        }
    };
    loading.frames_left = loading.frames_left.saturating_sub(1);
    if loading.frames_left == 0 {
        commands.entity(eid).remove::<ConvoBoxRootLoading>();
        match speaker {
            ConvoSpeaker::Silence(_) => {}
            _ => {
                *viz = Visibility::Inherited;
            }
        }
        commands.entity(eid).insert(ConvoTextStarted);
        for oid in &stale {
            commands.entity(oid).despawn_recursive();
        }
    }
}

fn update_managers(
    mut manager_q: Query<(Entity, &mut ConvoManager)>,
    mut boxes_q: Query<(
        Entity,
        &ConvoBoxRoot,
        Option<&ConvoTextStarted>,
        Option<&ConvoTextDone>,
        &mut ConvoSpeaker,
    )>,
    mut commands: Commands,
    butt: Res<ButtInput>,
    time: Res<Time>,
) {
    let (eid, mut manager) = match manager_q.get_single_mut() {
        Ok(manager) => manager,
        Err(QuerySingleError::NoEntities(_)) => return,
        Err(QuerySingleError::MultipleEntities(e)) => panic!("ruh roh update convo managers {e:?}"),
    };
    if boxes_q.is_empty() {
        match manager.boxes.pop() {
            Some(boox) => {
                commands.spawn(boox);
            }
            None => {
                commands.entity(eid).despawn_recursive();
            }
        }
    } else {
        let any_loading = boxes_q
            .iter()
            .any(|multi| multi.2.is_none() && multi.3.is_none());
        if any_loading {
            return;
        }

        let special_silence = boxes_q
            .iter_mut()
            .filter(|multi| {
                multi.2.is_none()
                    && multi.3.is_some()
                    && matches!(multi.4.as_ref(), ConvoSpeaker::Silence(_))
            })
            .next();
        match special_silence {
            Some(mut multi) => match multi.4.as_mut() {
                ConvoSpeaker::Silence(inner) => {
                    inner.sub_assign(time.delta_seconds());
                    if *inner <= 0.0 {
                        if let Some(next_box) = manager.boxes.pop() {
                            commands.spawn(next_box);
                        } else {
                            for multi in &boxes_q {
                                commands.entity(multi.0).despawn_recursive();
                            }
                        }
                        return;
                    }
                }
                _ => {}
            },
            None => {}
        }

        let started = boxes_q
            .iter()
            .filter(|multi| multi.2.is_some())
            .collect::<Vec<_>>();
        let done = boxes_q
            .iter()
            .filter(|multi| multi.3.is_some())
            .collect::<Vec<_>>();

        let has_skip_input = butt.just_pressed(ButtKind::Enter);
        if has_skip_input {
            for multi in &boxes_q {
                commands.entity(multi.0).despawn_recursive();
            }
            manager.boxes.clear();
            return;
        }

        let has_forward_input = butt.just_pressed(ButtKind::A);
        if !has_forward_input {
            return;
        }

        match (started.is_empty(), done.is_empty()) {
            (true, true) => unreachable!(),
            (false, _) => {
                // We want to finish all the texts marked as started
                for multi in started {
                    commands.entity(multi.0).insert(ConvoTextDone);
                }
            }
            (true, false) => {
                if let Some(next_box) = manager.boxes.pop() {
                    commands.spawn(next_box);
                } else {
                    for multi in &boxes_q {
                        commands.entity(multi.0).despawn_recursive();
                    }
                }
            }
        }
    }
}

pub(super) fn register_convo_box(app: &mut App) {
    reg_types!(app, ConvoSpeaker);
    app.add_systems(Update, (finish_loading_box, update_managers));
}
