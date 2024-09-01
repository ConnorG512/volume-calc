mod input;
mod volprop;

fn main() {

    let mut input_volume = volprop::Volume {
        length_x: 0.0,
        length_y: 0.0,
        length_z: 0.0
    };

    println!("Please enter the volume for x:");
    input_volume.length_x = input::take_user_input();

    println!("Please enter the volume for y:");
    input_volume.length_y = input::take_user_input();
    
    println!("Please enter the volume for z:");
    input_volume.length_z = input::take_user_input();

    let result: f64 = calculate_value(&mut input_volume.length_x, &mut input_volume.length_y, &mut input_volume.length_z);
    print_result(result)
}

fn calculate_value(val_x: &mut f64, val_y: &mut f64, val_z: &mut f64) -> f64 {
    let calculated_value: f64 = *val_x * *val_y * *val_z;

    return calculated_value;
}

fn print_result(result: f64) {
    let text: &str = &result.to_string();
    println!("The value of the volume is: {text}");
}
