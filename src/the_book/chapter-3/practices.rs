fn main() {
    let celsius_temp = convert_farenheit_to_celsius(72.1);
    println!("{celsius_temp}");
    println!();
}

fn convert_farenheit_to_celsius(farenheit_temp: f32) -> f32 {
    (farenheit_temp - 32.0) * (5.0/9.0)
}