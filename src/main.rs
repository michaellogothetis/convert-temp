// Exercises from 'The Rust Programming Language'
// https://doc.rust-lang.org/book/ch03-05-control-flow.html#summary
//
// A program to convert between degrees Celsius and degrees Fahrenheit
//
// Author: Michael Logothetis
// Date: 06-Sep-2022
//
// Let's expand this handle temperature conversion more generally.
//
// This was a learning exercise. If you need a way to handle temperature
// and other measurments in Rust, look at the measurements crate (https://docs.rs/measurements/latest/measurements/#).
//
// Celsius to:
//   Fahrenheit: degreesF = degreesC * 1.8 + 32.0
//   kelvin: K = degreesC + 273.15
//
// Fahrenheit to:
//   Celsius: degreesC = (degreesF - 32.0) / 1.8
//   kelvin: K = (degreesF - 32.0) / 1.8 + 273.15
//
// kelvin to:
//   Celsius - degreesC = K - 273.15
//   Fahrenheit - degreesF = (K - 273.15) * 1.8 + 32.0

// temperature = Temp(temperature, units = "C")
// temperature_fahrenheit = temperature.to_fahrenheit()
// temperature_kelvin = temperature.to_kelvin()
// temperature_celsius = temperature_kelvin.to_celsius()
//
// temp_celsius = to_celsius(temp_fahrenheit)
// temp_fahrenheit = to_fahrenheit(temp_celsius)
// converted_temp = convert_temp(temp) temp: 37.5C or 99.5F

use std::env;
use crate::temperature::Temperature;
use crate::temperature::TemperatureUnit;
use crate::temperature::{ABSOLUTE_ZERO, BOILING_POINT, FREEZING_POINT};

use sys_locale::get_locale;

pub mod temperature;

fn main() {

    let args: Vec<String> = env::args().collect();
    dbg!(args);

    let locale = get_locale().unwrap_or_else(|| String::from("en-US"));

    println!("The current locale is {}", locale);

    //let absolute_zero_kelvin = Temperature::new(0.0, TemperatureUnit::Kelvin);
    let _absolute_zero_kelvin = match Temperature::new(ABSOLUTE_ZERO.value, ABSOLUTE_ZERO.unit) {
        Ok(temp) => temp,
        Err(err) => {
            eprintln!("Invalid temperature: {err}");
            return;
        }
    };

    let boiling_point_celsius = match Temperature::new(BOILING_POINT.value, BOILING_POINT.unit) {
        Ok(temp) => temp,
        Err(err) => {
            eprintln!("Invalid temperature: {err}");
            return;
        }
    };

    let boiling_point_fahrenheit= boiling_point_celsius.to(TemperatureUnit::Fahrenheit);
    println!("The boiling point of water at sea level is {boiling_point_celsius} or {boiling_point_fahrenheit}");

    let freezing_point_celsius = match Temperature::new(FREEZING_POINT.value, FREEZING_POINT.unit) {
        Ok(temp) => temp,
        Err(err) => {
            eprintln!("Invalid temperature: {err}");
            return;
        }
    };

    let freezing_point_fahrenheit = freezing_point_celsius.to(TemperatureUnit::Fahrenheit);
    let freezing_point_kelvin = freezing_point_celsius.to(TemperatureUnit::Kelvin);

    println!("The temperature at which water freezes can be expressed as {freezing_point_celsius}, {freezing_point_fahrenheit} or {freezing_point_kelvin}");

}
