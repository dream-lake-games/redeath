use crate::prelude::*;

// Often nice to have the size of the screen in these formats
pub const SCREEN_WIDTH: u32 = 240;
pub const SCREEN_HEIGHT: u32 = 180;
pub const SCREEN_UVEC: UVec2 = UVec2::new(SCREEN_WIDTH, SCREEN_HEIGHT);
#[allow(nonstandard_style)]
pub const SCREEN_WIDTH_f32: f32 = SCREEN_WIDTH as f32;
#[allow(nonstandard_style)]
pub const SCREEN_HEIGHT_f32: f32 = SCREEN_HEIGHT as f32;
pub const SCREEN_VEC: Vec2 = Vec2::new(SCREEN_WIDTH_f32, SCREEN_HEIGHT_f32);

// The actual window resolution should be bigger than screen width, so menus (and especially text) have more detail
pub const WINDOW_GROWTH: u32 = 6;
pub const WINDOW_WIDTH: u32 = SCREEN_WIDTH * WINDOW_GROWTH;
pub const WINDOW_HEIGHT: u32 = SCREEN_HEIGHT * WINDOW_GROWTH;
pub const WINDOW_UVEC: UVec2 = UVec2::new(WINDOW_WIDTH, WINDOW_HEIGHT);
#[allow(nonstandard_style)]
pub const WINDOW_WIDTH_f32: f32 = WINDOW_WIDTH as f32;
#[allow(nonstandard_style)]
pub const WINDOW_HEIGHT_f32: f32 = WINDOW_HEIGHT as f32;
pub const WINDOW_VEC: Vec2 = Vec2::new(WINDOW_WIDTH_f32, WINDOW_HEIGHT_f32);

// This is frames per second by the way
pub const FRAMERATE: f32 = 36.0;
pub const DEFAULT_ANIMATION_FPS: f32 = 16.0;

// Keeping constant ZIX's here makes debugging weird layering stuff MUCH easier
pub const ZIX_MAX: f32 = 600.0; // Anything further forward than this gets culled by the camera(s)
pub const ZIX_MENU: i32 = 100;
pub const ZIX_TRANSITION: i32 = 200;
pub const ZIX_MIN: f32 = -600.0; // Anything further back than this gets culled by the camera(s)

// Colors are easy!
pub const COLOR_NONE: Color = Color::linear_rgba(0.0, 0.0, 0.0, 0.0);
