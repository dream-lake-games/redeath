use bevy::sprite::Material2dPlugin;

use crate::prelude::*;

pub mod palette_mat;

pub use palette_mat::*;

pub fn color_as_vec4(color: Color) -> Vec4 {
    let linear = color.to_linear();
    Vec4::new(linear.red, linear.green, linear.blue, 1.0)
}

#[derive(Resource, Clone, Debug, Reflect)]
pub struct Palette {
    pub zero: Color,
    pub one: Color,
    pub two: Color,
    pub three: Color,
    pub four: Color,
    pub five: Color,
    pub six: Color,
    pub seven: Color,
}
impl std::ops::Index<i32> for Palette {
    type Output = Color;
    fn index(&self, ix: i32) -> &Self::Output {
        if ix < 0 {
            return &self.zero;
        }
        if ix > 7 {
            return &self.seven;
        }
        match ix {
            0 => &self.zero,
            1 => &self.one,
            2 => &self.two,
            3 => &self.three,
            4 => &self.four,
            5 => &self.five,
            6 => &self.six,
            7 => &self.seven,
            _ => unreachable!(),
        }
    }
}
impl Default for Palette {
    fn default() -> Self {
        BERRY_NEBULA.clone()
    }
}

pub const BERRY_NEBULA: Palette = Palette {
    zero: Color::linear_rgb(13.0 / 255.0, 0.0 / 255.0, 26.0 / 255.0),
    one: Color::linear_rgb(46.0 / 255.0, 10.0 / 255.0, 48.0 / 255.0),
    two: Color::linear_rgb(79.0 / 255.0, 20.0 / 255.0, 70.0 / 255.0),
    three: Color::linear_rgb(111.0 / 255.0, 29.0 / 255.0, 92.0 / 255.0),
    four: Color::linear_rgb(110.0 / 255.0, 81.0 / 255.0, 129.0 / 255.0),
    five: Color::linear_rgb(109.0 / 255.0, 133.0 / 255.0, 165.0 / 255.0),
    six: Color::linear_rgb(108.0 / 255.0, 185.0 / 255.0, 201.0 / 255.0),
    seven: Color::linear_rgb(108.0 / 255.0, 237.0 / 255.0, 237.0 / 255.0),
};

pub const WAVERATOR: Palette = Palette {
    zero: Color::linear_rgb(12.0 / 255.0, 13.0 / 255.0, 20.0 / 255.0),
    one: Color::linear_rgb(24.0 / 255.0, 28.0 / 255.0, 40.0 / 255.0),
    two: Color::linear_rgb(35.0 / 255.0, 49.0 / 255.0, 61.0 / 255.0),
    three: Color::linear_rgb(51.0 / 255.0, 80.0 / 255.0, 93.0 / 255.0),
    four: Color::linear_rgb(78.0 / 255.0, 127.0 / 255.0, 125.0 / 255.0),
    five: Color::linear_rgb(83.0 / 255.0, 167.0 / 255.0, 136.0 / 255.0),
    six: Color::linear_rgb(112.0 / 255.0, 211.0 / 255.0, 139.0 / 255.0),
    seven: Color::linear_rgb(203.0 / 255.0, 205.0 / 255.0, 216.0 / 255.0),
};

pub(super) struct PalettePlugin;
impl Plugin for PalettePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WAVERATOR.clone());
        app.add_plugins(Material2dPlugin::<PaletteMat>::default());
    }
}
