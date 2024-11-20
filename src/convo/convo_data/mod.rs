use crate::prelude::*;

pub mod convo_canyon;

pub use convo_canyon::*;

pub(super) fn register_convo_data(app: &mut App) {
    app.add_plugins(ConvoCanyonPlugin::new());
}

pub struct ConvoDataPlugin<T: ConvoKind> {
    _pd: std::marker::PhantomData<T>,
}
impl<T: ConvoKind> ConvoDataPlugin<T> {
    pub fn new() -> Self {
        Self {
            _pd: std::marker::PhantomData,
        }
    }
}

macro_rules! decl_convo {
    (
        $name:ident,
        $(
            (
                $variant:ident,
                [$(
                    {
                        speaker: $speaker:expr,
                        portrait: $portrait:literal,
                        sound: $sound:literal,
                        text: $text:literal$(,)?
                    }$(,)?
                )+]$(,)?
            )
            $(,)?
        )+
    ) => {
        paste::paste! {
        #[derive(Clone, Copy, Debug, Reflect, Hash, PartialEq, Eq, States)]
        pub enum $name {
            $(
                $variant,
            )+
        }
        impl Queryable for $name {}
        impl ConvoKind for $name {
            fn get_convo_boxes(&self) -> Vec<ConvoBox> {
                match self {
                    $(
                        Self::$variant => vec![
                            $(
                                ConvoBox::new($speaker, $portrait, $sound, ConvoText::simple($text)),
                            )+
                        ],
                    )+
                }
            }
        }
            /// For triggering the convo
            #[derive(Event)]
            pub enum [<Start $name>] {
                $(
                    $variant,
                )+
            }
            impl [<Start $name>] {
                fn to_base_variant(&self) -> $name {
                    match self {
                        $(
                            Self::$variant => $name::$variant,
                        )+
                    }
                }
            }
            #[allow(nonstandard_style)]
            fn [<observe_start_$name>](
                trigger: Trigger<[<Start $name>]>,
                convo_root: Res<ConvoRoot>,
                mut commands: Commands,
                mut next_state: ResMut<NextState<ConvoMetaState>>,
            ) {
                let variant = trigger.event().to_base_variant();

                // World management
                commands.entity(convo_root.eid()).despawn_descendants();
                commands.spawn((
                    Name::new("convo_manager"),
                    ConvoManager::new(variant.get_convo_boxes().into_iter()),
                    [<$name Sentinel>]::from_base_variant(variant),
                )).set_parent(convo_root.eid());

                // Trigger and state
                next_state.set(ConvoMetaState::Some);
                commands.trigger([<$name Started>]::from_base_variant(variant.clone()));
            }

            /// For reacting to the convo
            #[derive(Event, Debug)]
            pub enum [<$name Started>] {
                $(
                    $variant,
                )+
            }
            impl [<$name Started>] {
                fn from_base_variant(variant: $name) -> Self {
                    match variant {
                        $(
                            $name::$variant => Self::$variant,
                        )+
                    }
                }
            }
            /// For reacting to the convo
            #[derive(Event, Debug)]
            pub enum [<$name Ended>] {
                $(
                    $variant,
                )+
            }
            impl [<$name Ended>] {
                fn from_base_variant(variant: $name) -> Self {
                    match variant {
                        $(
                            $name::$variant => Self::$variant,
                        )+
                    }
                }
            }

            enum [<$name Sentinel>] {
                $(
                    $variant,
                )+
            }
            impl [<$name Sentinel>] {
                fn to_base_variant(&self) -> $name {
                    match self {
                        $(
                            Self::$variant => $name::$variant,
                        )+
                    }
                }
                fn from_base_variant(variant: $name) -> Self {
                    match variant {
                        $(
                            $name::$variant => Self::$variant,
                        )+
                    }
                }
            }
            impl Component for [<$name Sentinel>] {
                const STORAGE_TYPE: StorageType = StorageType::Table;
                fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
                    hooks.on_remove(|mut world, eid, _| {
                        let mut next_state = world.resource_mut::<NextState<ConvoMetaState>>();
                        next_state.set(ConvoMetaState::None);
                        let myself = world.get::<Self>(eid).expect("myself").to_base_variant();
                        world.commands().trigger([<$name Ended>]::from_base_variant(myself));
                        println!("should have triggered end...");
                    });
                }
            }

            impl<T: ConvoKind> Plugin for ConvoDataPlugin<T> {
                fn build(&self, app: &mut App) {
                    app.observe([<observe_start_$name>]);

                }
            }
            pub(super) type [<$name Plugin>] = ConvoDataPlugin<$name>;
        }
    };
}
use decl_convo;

use super::ConvoKind;
