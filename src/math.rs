use crate::prelude::*;

pub fn closest_card_dir(v: Vec2) -> CardDir {
    let ang = v.to_angle();
    const PI: f32 = std::f32::consts::PI;
    if -PI / 8.0 <= ang && ang < PI / 8.0 {
        CardDir::E
    } else if PI / 8.0 <= ang && ang < 3.0 * PI / 8.0 {
        CardDir::NE
    } else if 3.0 * PI / 8.0 <= ang && ang < 5.0 * PI / 8.0 {
        CardDir::N
    } else if 5.0 * PI / 8.0 <= ang && ang < 7.0 * PI / 8.0 {
        CardDir::NW
    } else if 7.0 * PI / 8.0 <= ang || ang <= -7.0 * PI / 8.0 {
        CardDir::W
    } else if -5.0 * PI / 8.0 >= ang && ang > -7.0 * PI / 8.0 {
        CardDir::SW
    } else if -3.0 * PI / 8.0 >= ang && ang > -5.0 * PI / 8.0 {
        CardDir::S
    } else if -PI / 8.0 >= ang && ang > -3.0 * PI / 8.0 {
        CardDir::SE
    } else {
        warn!("Bad closest_card_dir");
        CardDir::E
    }
}

pub trait MyPick<V> {
    fn pick(&self) -> V;
}
impl<V: Clone> MyPick<V> for Vec<V> {
    fn pick(&self) -> V {
        self.choose(&mut thread_rng()).unwrap().clone()
    }
}
