use crate::prelude::*;

#[derive(Resource, Default)]
pub struct GlobalPaletteShift {
    active_shifts: Vec<(f32, i32)>,
}
impl GlobalPaletteShift {
    pub fn add(&mut self, time: f32, diff: i32) {
        self.active_shifts.push((time, diff));
    }
    pub fn purge(&mut self, time: f32) {
        self.active_shifts.iter_mut().for_each(|el| el.0 -= time);
        self.active_shifts.retain(|el| el.0 > 0.0);
    }
    pub fn reduce(&self) -> i32 {
        self.active_shifts
            .iter()
            .map(|el| el.1)
            .reduce(|acc, el| acc + el)
            .unwrap_or(0)
    }
}

fn update_global_shift(
    time: Res<Time>,
    mut shifts: ResMut<GlobalPaletteShift>,
    res_palette: Res<Palette>,
    wavy_palette_manager: Res<WavyPaletteManager>,
    hands: Query<&MeshMaterial2d<ShiftedPaletteMat>>,
    mut mats: ResMut<Assets<ShiftedPaletteMat>>,
) {
    shifts.purge(time.delta_secs());
    let shift_amt = shifts.reduce();
    let maybe_wavy_palette = wavy_palette_manager.calc();
    let palette = maybe_wavy_palette.as_ref().unwrap_or(&res_palette);
    for hand in &hands {
        let mat = mats.get_mut(hand.id()).unwrap();
        mat.take_shifted_palette(shift_amt, palette);
    }
}

pub(super) fn register_global_shift(app: &mut App) {
    app.insert_resource(GlobalPaletteShift::default());
    app.add_systems(Update, update_global_shift);
}
