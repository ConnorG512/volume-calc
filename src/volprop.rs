pub struct Volume {
    pub length_x: f64,
    pub length_y: f64,
    pub length_z: f64,
}

pub fn calculate_value(val_x: &mut f64, val_y: &mut f64, val_z: &mut f64) -> f64 {
    let calculated_value: f64 = *val_x * *val_y * *val_z;

    return calculated_value;
}