//! It's pet peeve of mine to have a disorganized hierarchy in the debugger.
//! It looks bad, and when stuff goes wrong it makes it much harder to actually
//! figure out what's wrong.
//! Also it's good to have invariants like: Everything that should be despawned when going back to menu
//! exists under the same ancestor, so .despawn_descendents works fine

use crate::prelude::*;

pub trait RootKind: Resource {
    fn eid(&self) -> Entity;
}

macro_rules! impl_root_types {
    ($name:ident) => {
        paste::paste! {
            #[derive(Component, Debug, Reflect)]
            pub struct[<$name Component>];

            #[derive(Bundle)]
            pub struct[<$name Bundle>] {
                name: Name,
                marker: [<$name Component>],
                transform: Transform,
                visibility: Visibility,
            }
            impl [<$name Bundle>] {
                fn new(zix: f32) -> Self {
                    Self {
                        name: Name::new(stringify!($name)),
                        marker: [<$name Component>],
                        transform: Transform::from_translation(Vec2::ZERO.extend(zix)),
                        visibility: Visibility::Inherited,
                    }
                }
            }

            #[derive(Resource, Debug, Reflect)]
            pub struct $name {
                eid: Entity,
            }
            impl RootKind for $name {
                fn eid(&self) -> Entity {
                    self.eid
                }
            }
        }
    };
}

macro_rules! impl_root_init {
    ($($name:ident$(($zix:expr))?),* $(,)?) => {
        $(
            impl_root_types!($name);
        )*

        paste::paste! {
            fn setup_roots(
                mut commands: Commands,
                $(
                    #[allow(nonstandard_style)]
                    mut [<$name _res>]: ResMut<$name>,
                )*
            ) {
                $(
                    #[allow(unused_mut, unused_assignments)]
                    let mut zix = i32::default();
                    $(
                        zix = $zix;
                    )?
                    #[allow(nonstandard_style)]
                    let [<$name _eid>] = commands.spawn([<$name Bundle>]::new(zix as f32)).id();
                    [<$name _res>].eid = [<$name _eid>];
                )*
            }
        }

        #[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
        pub struct RootInit;

        fn cleanup_observes(
            mut commands: Commands,
            ents: Query<Entity, (With<bevy::ecs::observer::ObserverState>, Without<Parent>)>,
            root: Res<ObserveRoot>,
        ) {
            for eid in &ents {
                commands.entity(eid).set_parent(root.eid());
            }
        }

        pub(super) struct RootPlugin;
        impl Plugin for RootPlugin {
            fn build(&self, app: &mut App) {
                $(
                    app.insert_resource($name {
                        eid: Entity::PLACEHOLDER,
                    });
                )*
                insert_subroots(app);

                app.add_systems(Startup, (setup_roots, setup_subroots).chain().in_set(RootInit));
                app.add_systems(PreUpdate, cleanup_observes);
            }
        }
    };
}

macro_rules! impl_subroot_init {
    ($($name:ident($parent:ident)$(,)?)*) => {
        $(
            impl_root_types!($name);
        )*

        paste::paste! {
            fn setup_subroots(
                mut commands: Commands,
                $(
                    #[allow(nonstandard_style)]
                    mut [<$name _res>]: ResMut<$name>,
                    #[allow(nonstandard_style)]
                    [<$parent $name _res>]: Res<$parent>,
                )*
            ) {
                $(
                    #[allow(nonstandard_style)]
                    let [<$name _eid>] = commands.spawn([<$name Bundle>]::new(0.0)).set_parent([<$parent $name _res>].eid()).id();
                    [<$name _res>].eid = [<$name _eid>];
                )*
            }
        }


        fn insert_subroots(app: &mut App) {
            $(
                app.insert_resource($name {
                    eid: Entity::PLACEHOLDER,
                });
            )*
        }
    };
}

impl_root_init!(
    LayerRoot,
    WorldRoot,
    MenuRoot(ZIX_MENU),
    TransitionRoot(ZIX_TRANSITION),
    ObserveRoot,
    SoundRoot,
    ConvoRoot,
    CutsceneRoot,
    PauseRoot(ZIX_PAUSE),
    DebugRoot,
);

impl_subroot_init!(
    PlatformRoot(WorldRoot),
    WorldDetailRoot(WorldRoot),
    LightRoot(WorldRoot),
    BgFgRoot(WorldRoot),
    ItemsRoot(WorldRoot),
    WorldMetaRoot(WorldRoot),
);
