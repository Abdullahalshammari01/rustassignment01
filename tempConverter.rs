const FREEZING_FAHRENHEIT: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_FAHRENHEIT) * (5.0 / 9.0)
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * (9.0 / 5.0)) + FREEZING_FAHRENHEIT
}

fn main() {
    let mut fahrenheit_temp = 32.0;

    let celsius_temp = fahrenheit_to_celsius(fahrenheit_temp);
    println!("{:.2}°F is equal to {:.2}°C", fahrenheit_temp, celsius_temp);

    println!("Fahrenheit temperatures:");
    for _ in 0..5 {
        fahrenheit_temp += 1.0;
        let celsius_temp = fahrenheit_to_celsius(fahrenheit_temp);
        println!("{:.2}°F is equal to {:.2}°C", fahrenheit_temp, celsius_temp);
    }



}
