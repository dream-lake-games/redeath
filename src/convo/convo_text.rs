use crate::prelude::*;

#[derive(Clone, Debug, Reflect)]
pub struct ConvoText {
    pub text: String,
}
impl ConvoText {
    pub fn simple(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

#[derive(Component, Clone, Debug, Reflect)]
pub struct ConvoTextStarted;
#[derive(Clone, Debug, Reflect)]
pub struct ConvoTextDone;

#[derive(Component, Clone, Debug, Reflect)]
struct ConvoTextProgress {
    time_till_next_word: f32,
    shown: String,
    words_left: Vec<String>,
}
impl ConvoTextProgress {
    const fn word_time() -> f32 {
        0.06
    }

    fn new(content: String) -> Self {
        Self {
            time_till_next_word: Self::word_time(),
            shown: String::new(),
            words_left: content
                .split_whitespace()
                .map(|t| t.to_string())
                .rev()
                .collect(),
        }
    }

    fn show_next_word(&mut self) -> bool {
        match self.words_left.pop() {
            Some(word) => {
                self.shown += &word;
                self.shown += " ";
                self.time_till_next_word = Self::word_time();
                true
            }
            None => false,
        }
    }

    fn finish(&mut self) {
        let remaining_words: Vec<_> = self.words_left.drain(..).rev().collect();
        self.shown += &remaining_words.join(" ");
    }
}

impl Component for ConvoText {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            let content = world.get::<Self>(eid).unwrap().text.clone();
            let font_hand = world
                .resource::<AssetServer>()
                .load("fonts/KodeMono/KodeMono-Medium.ttf");
            world
                .commands()
                .spawn((
                    Name::new("text"),
                    ConvoTextProgress::new(content),
                    Text2d::new(String::new()),
                    TextFont::from_font(font_hand).with_font_size(32.0),
                    TextLayout::new_with_justify(JustifyText::Left),
                    TextBounds::new(860.0, 160.0),
                    Anchor::CenterLeft,
                    Transform::from_translation(Vec3::new(-336.0, 0.0, ZIX_CONVO_TEXT)),
                    Visibility::Inherited,
                    TextLayer::to_render_layers(),
                ))
                .set_parent(eid);
        });
    }
}

impl Component for ConvoTextDone {
    const STORAGE_TYPE: StorageType = StorageType::Table;
    fn register_component_hooks(hooks: &mut bevy::ecs::component::ComponentHooks) {
        hooks.on_add(|mut world, eid, _| {
            let children = world
                .get::<Children>(eid)
                .expect("my children")
                .into_iter()
                .map(|e| e.clone())
                .collect::<Vec<_>>();
            for child in children {
                let Some(mut progress) = world.get_mut::<ConvoTextProgress>(child) else {
                    continue;
                };
                progress.finish();
            }
            world.commands().entity(eid).remove::<ConvoTextStarted>();
        });
    }
}

fn update_text(
    started_q: Query<&ConvoTextStarted>,
    mut progress: Query<(&Parent, &mut ConvoTextProgress)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (parent, mut progress) in &mut progress {
        if !started_q.contains(parent.get()) {
            continue;
        }
        progress.time_till_next_word -= time.delta_secs();
        if progress.time_till_next_word > 0.0 {
            continue;
        }
        if !progress.show_next_word() {
            commands.entity(parent.get()).insert(ConvoTextDone);
        }
    }
}

fn show_text(mut text_q: Query<(&mut Text2d, &ConvoTextProgress)>) {
    for (mut text, progress) in &mut text_q {
        text.0 = progress.shown.clone();
    }
}

pub(super) fn register_convo_text(app: &mut App) {
    app.add_systems(Update, (update_text, show_text));
}
