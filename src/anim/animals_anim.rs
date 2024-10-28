use crate::prelude::*;

derive_anim!(
    pub enum FireflyAnim {
        #[default]
        #[file("environment/animals/firefly_rest_ground.png")]
        #[size(8, 8)]
        RestGround,
        #[file("environment/animals/firefly_rest_wall.png")]
        #[size(8, 8)]
        RestWall,
        #[file("environment/animals/firefly_flap.png")]
        #[size(8, 8)]
        #[length(4)]
        Flap,
    }
);
type FireflyAnimPlugin = AnimDefnPlugin<FireflyAnim, AnimTimeRes>;

pub(super) fn register_animals_anim(app: &mut App) {
    app.add_plugins(FireflyAnimPlugin::default());
}
