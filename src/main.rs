//! `convert-temp` command-line tool.
//!
//! Convert temperatures between Celsius, Fahrenheit, and Kelvin.
//!
//! # Usage
//! ```text
//! convert-temp <value> <from_unit> <to_unit>
//! ```
//!
//! Units are `C`, `F`, or `K`.
//!
//! # Examples
//! ```text
//! convert-temp 37.5 C F
//! convert-temp 273.15 K C
//! ```
//!
//! # Flags
//! - `-h`, `--help` show usage
//! - `-V`, `--version` show version
//!
//! This tool is a learning exercise; see `temperature` module for the core API.
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
    if args.len() == 2 && (args[1] == "-h" || args[1] == "--help") {
        println!("Usage: {} <value> <from_unit> <to_unit>", args[0]);
        println!("Units: C, F, K");
        return;
    }

    if args.len() == 2 && (args[1] == "-V" || args[1] == "--version") {
        println!("{} {}", args[0], env!("CARGO_PKG_VERSION"));
        return;
    }

    if args.len() != 4 {
        eprintln!("Usage: {} <value> <from_unit> <to_unit>", args[0]);
        eprintln!("Units: C, F, K");
        return;
    }

    let locale = get_locale().unwrap_or_else(|| String::from("en-US"));
    println!("The current locale is {}", locale);

    let from_value: f64 = match args[1].parse() {
        Ok(value) => value,
        Err(_) => {
            eprintln!("Invalid temperature value: {}", args[1]);
            return;
        }
    };

    let from_unit = match args[2].as_str() {
        "C" => TemperatureUnit::Celsius,
        "F" => TemperatureUnit::Fahrenheit,
        "K" => TemperatureUnit::Kelvin,
        _ => {
            eprintln!("Invalid from unit: {}", args[2]);
            eprintln!("Units: C, F, K");
            return;
        }
    };

    let to_unit = match args[3].as_str() {
        "C" => TemperatureUnit::Celsius,
        "F" => TemperatureUnit::Fahrenheit,
        "K" => TemperatureUnit::Kelvin,
        _ => {
            eprintln!("Invalid to unit: {}", args[3]);
            eprintln!("Units: C, F, K");
            return;
        }
    };

    let from_temp = match Temperature::new(from_value, from_unit) {
        Ok(temp) => temp,
        Err(err) => {
            eprintln!("Invalid temperature: {err}");
            return;
        }
    };

    let to_temp = from_temp.to(to_unit);
    println!("{from_temp} = {to_temp}");

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

    let boiling_point_fahrenheit = boiling_point_celsius.to(TemperatureUnit::Fahrenheit);
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
