use core::f32;

use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct PaletteWave {
    offset: f32,
    mag: f32,
    period: f32,
}
impl PaletteWave {
    pub fn new(offset: f32, mag: f32, period: f32) -> Self {
        Self {
            offset,
            mag,
            period,
        }
    }
}

pub struct WavyPalette {
    pub base: Palette,
    pub r_wave: PaletteWave,
    pub g_wave: PaletteWave,
    pub b_wave: PaletteWave,
}
impl WavyPalette {
    pub fn new(base: Palette) -> Self {
        Self {
            base,
            r_wave: PaletteWave::new(0.0, 0.0, 1.0),
            g_wave: PaletteWave::new(0.0, 0.0, 1.0),
            b_wave: PaletteWave::new(0.0, 0.0, 1.0),
        }
    }
    pub fn with_r(mut self, offset: f32, mag: f32, period: f32) -> Self {
        self.r_wave = PaletteWave::new(offset, mag, period);
        self
    }
    pub fn with_g(mut self, offset: f32, mag: f32, period: f32) -> Self {
        self.g_wave = PaletteWave::new(offset, mag, period);
        self
    }
    pub fn with_b(mut self, offset: f32, mag: f32, period: f32) -> Self {
        self.b_wave = PaletteWave::new(offset, mag, period);
        self
    }
}

#[derive(Resource, Default)]
pub struct WavyPaletteManager {
    inner: Option<(WavyPalette, f32)>,
}
impl WavyPaletteManager {
    pub fn set(&mut self, wp: WavyPalette) {
        self.inner = Some((wp, 0.0));
    }
    pub fn unset(&mut self) {
        self.inner = None;
    }

    pub fn calc(&self) -> Option<Palette> {
        let Some((wp, time)) = &self.inner else {
            return None;
        };
        let new = Palette {
            zero: Color::linear_rgb(
                wp.base.zero.to_linear().red
                    + wp.r_wave.offset
                    + wp.r_wave.mag * (time / wp.r_wave.period).sin(),
                wp.base.zero.to_linear().green
                    + wp.g_wave.offset
                    + wp.g_wave.mag * (time / wp.g_wave.period).sin(),
                wp.base.zero.to_linear().blue
                    + wp.b_wave.offset
                    + wp.b_wave.mag * (time / wp.b_wave.period).sin(),
            ),
            one: Color::linear_rgb(
                wp.base.one.to_linear().red
                    + wp.r_wave.offset
                    + wp.r_wave.mag * (time / wp.r_wave.period).sin(),
                wp.base.one.to_linear().green
                    + wp.g_wave.offset
                    + wp.g_wave.mag * (time / wp.g_wave.period).sin(),
                wp.base.one.to_linear().blue
                    + wp.b_wave.offset
                    + wp.b_wave.mag * (time / wp.b_wave.period).sin(),
            ),
            two: Color::linear_rgb(
                wp.base.two.to_linear().red
                    + wp.r_wave.offset
                    + wp.r_wave.mag * (time / wp.r_wave.period).sin(),
                wp.base.two.to_linear().green
                    + wp.g_wave.offset
                    + wp.g_wave.mag * (time / wp.g_wave.period).sin(),
                wp.base.two.to_linear().blue
                    + wp.b_wave.offset
                    + wp.b_wave.mag * (time / wp.b_wave.period).sin(),
            ),
            three: Color::linear_rgb(
                wp.base.three.to_linear().red
                    + wp.r_wave.offset
                    + wp.r_wave.mag * (time / wp.r_wave.period).sin(),
                wp.base.three.to_linear().green
                    + wp.g_wave.offset
                    + wp.g_wave.mag * (time / wp.g_wave.period).sin(),
                wp.base.three.to_linear().blue
                    + wp.b_wave.offset
                    + wp.b_wave.mag * (time / wp.b_wave.period).sin(),
            ),
            four: Color::linear_rgb(
                wp.base.four.to_linear().red
                    + wp.r_wave.offset
                    + wp.r_wave.mag * (time / wp.r_wave.period).sin(),
                wp.base.four.to_linear().green
                    + wp.g_wave.offset
                    + wp.g_wave.mag * (time / wp.g_wave.period).sin(),
                wp.base.four.to_linear().blue
                    + wp.b_wave.offset
                    + wp.b_wave.mag * (time / wp.b_wave.period).sin(),
            ),
            five: Color::linear_rgb(
                wp.base.five.to_linear().red
                    + wp.r_wave.offset
                    + wp.r_wave.mag * (time / wp.r_wave.period).sin(),
                wp.base.five.to_linear().green
                    + wp.g_wave.offset
                    + wp.g_wave.mag * (time / wp.g_wave.period).sin(),
                wp.base.five.to_linear().blue
                    + wp.b_wave.offset
                    + wp.b_wave.mag * (time / wp.b_wave.period).sin(),
            ),
            six: Color::linear_rgb(
                wp.base.six.to_linear().red
                    + wp.r_wave.offset
                    + wp.r_wave.mag * (time / wp.r_wave.period).sin(),
                wp.base.six.to_linear().green
                    + wp.g_wave.offset
                    + wp.g_wave.mag * (time / wp.g_wave.period).sin(),
                wp.base.six.to_linear().blue
                    + wp.b_wave.offset
                    + wp.b_wave.mag * (time / wp.b_wave.period).sin(),
            ),
            seven: Color::linear_rgb(
                wp.base.seven.to_linear().red
                    + wp.r_wave.offset
                    + wp.r_wave.mag * (time / wp.r_wave.period).sin(),
                wp.base.seven.to_linear().green
                    + wp.g_wave.offset
                    + wp.g_wave.mag * (time / wp.g_wave.period).sin(),
                wp.base.seven.to_linear().blue
                    + wp.b_wave.offset
                    + wp.b_wave.mag * (time / wp.b_wave.period).sin(),
            ),
        };
        Some(new)
    }
}

fn update_wavy_palette(
    mut wavy_palette_manager: ResMut<WavyPaletteManager>,
    bullet_time: Res<BulletTime>,
) {
    if let Some((_wp, time)) = &mut wavy_palette_manager.inner {
        *time += bullet_time.delta_seconds() * f32::consts::PI * 2.0;
    }
}

pub(super) fn register_wavy_palette(app: &mut App) {
    app.insert_resource(WavyPaletteManager::default());
    app.add_systems(Update, update_wavy_palette);
}
