use crate::prelude::*;

derive_anim!(
    pub enum FireflyAnim {
        #[default]
        #[file("environment/animals/firefly_rest_ground.png")]
        #[size(8, 8)]
        #[render_layers(MainDetailLayer)]
        RestGround,
        #[file("environment/animals/firefly_rest_wall.png")]
        #[size(8, 8)]
        #[render_layers(MainDetailLayer)]
        RestWall,
        #[file("environment/animals/firefly_flap.png")]
        #[size(8, 8)]
        #[length(4)]
        #[render_layers(MainDetailLayer)]
        Flap,
    }
);
type FireflyAnimPlugin = AnimDefnPlugin<FireflyAnim, AnimTimeRes>;

pub(super) fn register_animals_anim(app: &mut App) {
    app.add_plugins(FireflyAnimPlugin::default());
}
