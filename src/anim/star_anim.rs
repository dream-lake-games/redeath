use crate::prelude::*;

derive_anim!(
    pub enum Star3Anim {
        #[default]
        #[file("environment/stars/3_idle.png")]
        #[size(3, 3)]
        #[offset(0.5, 0.5)]
        #[render_layers(BgLayer)]
        Idle,
        #[file("environment/stars/3_grow.png")]
        #[size(3, 3)]
        #[offset(0.5, 0.5)]
        #[render_layers(BgLayer)]
        #[fps(1.0)]
        #[next(Bright)]
        Grow,
        #[file("environment/stars/3_bright.png")]
        #[size(3, 3)]
        #[offset(0.5, 0.5)]
        #[render_layers(BgLayer)]
        Bright,
        #[file("environment/stars/3_shrink.png")]
        #[size(3, 3)]
        #[offset(0.5, 0.5)]
        #[render_layers(BgLayer)]
        #[fps(1.0)]
        #[next(Idle)]
        Shrink,
    }
);
type Star3AnimPlugin = AnimDefnPlugin<Star3Anim, AnimTimeRes>;

derive_anim!(
    pub enum Star5Anim {
        #[default]
        #[file("environment/stars/5_idle.png")]
        #[size(5, 5)]
        #[offset(0.5, 0.5)]
        #[render_layers(BgLayer)]
        Idle,
        #[file("environment/stars/5_grow.png")]
        #[size(5, 5)]
        #[offset(0.5, 0.5)]
        #[render_layers(BgLayer)]
        #[fps(1.0)]
        #[next(Bright)]
        Grow,
        #[file("environment/stars/5_bright.png")]
        #[size(5, 5)]
        #[offset(0.5, 0.5)]
        #[render_layers(BgLayer)]
        Bright,
        #[file("environment/stars/5_shrink.png")]
        #[size(5, 5)]
        #[offset(0.5, 0.5)]
        #[render_layers(BgLayer)]
        #[fps(1.0)]
        #[next(Idle)]
        Shrink,
    }
);
type Star5AnimPlugin = AnimDefnPlugin<Star5Anim, AnimTimeRes>;

derive_anim!(
    pub enum Star7Anim {
        #[default]
        #[file("environment/stars/7_idle.png")]
        #[size(7, 7)]
        #[offset(0.5, 0.5)]
        #[render_layers(BgLayer)]
        Idle,
        #[file("environment/stars/7_grow.png")]
        #[size(7, 7)]
        #[offset(0.5, 0.5)]
        #[render_layers(BgLayer)]
        #[fps(1.0)]
        #[next(Bright)]
        Grow,
        #[file("environment/stars/7_bright.png")]
        #[size(7, 7)]
        #[offset(0.5, 0.5)]
        #[render_layers(BgLayer)]
        Bright,
        #[file("environment/stars/7_shrink.png")]
        #[size(7, 7)]
        #[offset(0.5, 0.5)]
        #[render_layers(BgLayer)]
        #[fps(1.0)]
        #[next(Idle)]
        Shrink,
    }
);
type Star7AnimPlugin = AnimDefnPlugin<Star7Anim, AnimTimeRes>;

pub(super) fn register_star_anim(app: &mut App) {
    app.add_plugins(Star3AnimPlugin::default());
    app.add_plugins(Star5AnimPlugin::default());
    app.add_plugins(Star7AnimPlugin::default());
}
