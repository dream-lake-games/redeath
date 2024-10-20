use crate::prelude::*;

use super::parallax::ParallaxX;

const GROW_PROBABILITY: f32 = 0.001;
const SHRINK_PROBABILITY: f32 = 0.01;

trait DoStarAnimThings {
    fn do_star_things(&mut self);
}
macro_rules! impl_do_star_anim_things {
    ($($type:ty$(,)?)+) => {
        $(
            impl DoStarAnimThings for AnimMan<$type> {
                fn do_star_things(&mut self) {
                    match self.get_state() {
                        <$type>::Grow | <$type>::Shrink => return,
                        <$type>::Idle => {
                            if thread_rng().gen_bool(GROW_PROBABILITY as f64) {
                                self.set_state(<$type>::Grow);
                            }
                        }
                        <$type>::Bright => {
                            if thread_rng().gen_bool(SHRINK_PROBABILITY as f64) {
                                self.set_state(<$type>::Shrink);
                            }
                        }
                    }
                }
            }
        )+
    };
}
impl_do_star_anim_things!(Star7Anim);

#[derive(Bundle)]
struct StarBundle<StarAnim: AnimStateMachine> {
    name: Name,
    anim: AnimMan<StarAnim>,
    spatial: SpatialBundle,
    parallax: ParallaxX,
}
impl<StarAnim: AnimStateMachine> StarBundle<StarAnim> {
    fn new() -> Self {
        let mut rng = thread_rng();
        let wrap_size = Vec2::new(SCREEN_WIDTH_f32 * 2.0, SCREEN_HEIGHT_f32);
        let pos = Pos::new(
            rng.gen_range(0.0..wrap_size.x).round(),
            rng.gen_range(100.0..wrap_size.y).round(),
        )
        .translated(-wrap_size / 2.0);
        Self {
            name: Name::new("star"),
            anim: default(),
            spatial: pos.to_spatial(-10.0),
            parallax: ParallaxX::new(-rng.gen_range(0.005..0.008), wrap_size.x),
        }
    }
}

#[derive(Event)]
pub struct SpawnStarsEvent {
    pub num_stars: u32,
}
fn handle_spawn_stars_event(
    trigger: Trigger<SpawnStarsEvent>,
    mut commands: Commands,
    root: Res<BgFgRoot>,
) {
    let SpawnStarsEvent { num_stars } = trigger.event();
    for _ in 0..*num_stars {
        commands
            .spawn(StarBundle::<Star7Anim>::new())
            .set_parent(root.eid());
    }
}

fn update_bg_stars(mut sevens: Query<(&mut AnimMan<Star7Anim>, &mut Transform)>) {
    for (mut anim, mut tran) in &mut sevens {
        tran.translation = tran.translation.round();
        anim.do_star_things();
    }
}

pub(super) fn register_bg_stars(app: &mut App) {
    app.observe(handle_spawn_stars_event);

    app.add_systems(FixedUpdate, update_bg_stars);
}
