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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum Spleen {
    EaseInCubic,
    EaseOutCubic,
    EaseInOutCubic,
    EaseInQuad,
    EaseOutQuad,
    EaseInOutQuad,
    EaseInQuartic,
    EaseOutQuartic,
    EaseInOutQuartic,
    EaseInQuintic,
    EaseOutQuintic,
    EaseInOutQuintic,
}

impl Spleen {
    pub fn interp(&self, x: f32) -> f32 {
        match *self {
            Self::EaseInCubic => ease_in_cubic(x),
            Self::EaseOutCubic => ease_out_cubic(x),
            Self::EaseInOutCubic => ease_in_out_cubic(x),
            Self::EaseInQuad => ease_in_quad(x),
            Self::EaseOutQuad => ease_out_quad(x),
            Self::EaseInOutQuad => ease_in_out_quad(x),
            Self::EaseInQuartic => ease_in_quartic(x),
            Self::EaseOutQuartic => ease_out_quartic(x),
            Self::EaseInOutQuartic => ease_in_out_quartic(x),
            Self::EaseInQuintic => ease_in_quintic(x),
            Self::EaseOutQuintic => ease_out_quintic(x),
            Self::EaseInOutQuintic => ease_in_out_quintic(x),
        }
    }

    /// Given progress x, interps between min and max using this spleen
    pub fn bound_interp(&self, x: f32, min: f32, max: f32) -> f32 {
        min + self.interp(x) * (max - min)
    }
}

fn ease_in_cubic(x: f32) -> f32 {
    x * x * x
}

fn ease_out_cubic(x: f32) -> f32 {
    1.0 - ease_in_cubic(1.0 - x)
}

fn ease_in_out_cubic(x: f32) -> f32 {
    if x < 0.5 {
        4.0 * x * x * x
    } else {
        1.0 - ((0.0 - 2.0) * x + 2.0).powf(3.0) / 2.0
    }
}

fn ease_in_quad(x: f32) -> f32 {
    x * x
}

fn ease_out_quad(x: f32) -> f32 {
    1.0 - ease_in_quad(1.0 - x)
}

fn ease_in_out_quad(x: f32) -> f32 {
    if x < 0.5 {
        2.0 * x * x
    } else {
        1.0 - ((0.0 - 2.0) * x + 2.0).powf(2.0) / 2.0
    }
}

fn ease_in_quartic(x: f32) -> f32 {
    x * x * x * x
}

fn ease_out_quartic(x: f32) -> f32 {
    1.0 - ease_in_quartic(1.0 - x)
}

fn ease_in_out_quartic(x: f32) -> f32 {
    if x < 0.5 {
        8.0 * x.powi(4)
    } else {
        1.0 - ((-2.0 * x + 2.0).powi(4)) / 2.0
    }
}

fn ease_in_quintic(x: f32) -> f32 {
    x * x * x * x * x
}

fn ease_out_quintic(x: f32) -> f32 {
    1.0 - ease_in_quintic(1.0 - x)
}

fn ease_in_out_quintic(x: f32) -> f32 {
    if x < 0.5 {
        16.0 * x.powi(5)
    } else {
        1.0 - ((-2.0 * x + 2.0).powi(5)) / 2.0
    }
}
