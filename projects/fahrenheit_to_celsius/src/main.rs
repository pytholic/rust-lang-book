fn main() {
    let temp_celsius = fahrenheit_to_celsius(99.0);
    println!("Temperature in celsius is: {temp_celsius}")
}

fn fahrenheit_to_celsius(temp_fahrenheit: f32) -> f32 {
    (temp_fahrenheit - 32.0) * (5.0 / 9.0)
}