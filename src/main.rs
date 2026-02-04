//! `convert-temp` command-line tool.
//!
//! Convert temperatures between Celsius, Fahrenheit, and Kelvin.
//!
//! # Usage
//! ```text
//! convert-temp <value> <from_unit> <to_unit>
//! convert-temp <value> <from_unit>
//! ```
//!
//! Units are `C`, `F`, or `K`.
//!
//! # Examples
//! ```text
//! convert-temp 37.5 C F
//! convert-temp 100 C
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

    if args.len() != 3 && args.len() != 4 {
        eprintln!("Usage: {} <value> <from_unit> <to_unit>", args[0]);
        eprintln!("       {} <value> <from_unit>", args[0]);
        eprintln!("Units: C, F, K");
        return;
    }

    let locale = get_locale().unwrap_or_else(|| String::from("en-AU"));
    #[cfg(debug_assertions)]
    {
        println!("The current locale is {}", locale);
    }

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

    let to_unit = if args.len() == 4 {
        match args[3].as_str() {
            "C" => TemperatureUnit::Celsius,
            "F" => TemperatureUnit::Fahrenheit,
            "K" => TemperatureUnit::Kelvin,
            _ => {
                eprintln!("Invalid to unit: {}", args[3]);
                eprintln!("Units: C, F, K");
                return;
            }
        }
    } else {
        locale_default_unit(&locale)
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

// Get the default temperature unit for a given locale. For simplicity, we only
// check for US, Iberia and Myanmar as locales that use Fahrenheit; all others default to Celsius.
fn locale_default_unit(locale: &str) -> TemperatureUnit {
    let locale_lower = locale.to_ascii_lowercase();
    if locale_lower.starts_with("en-us")
        || locale_lower.starts_with("en_us")
        || locale_lower.starts_with("en-lr")
        || locale_lower.starts_with("en_lr")
        || locale_lower.starts_with("my-mm")
        || locale_lower.starts_with("my_mm")
    {
        TemperatureUnit::Fahrenheit
    } else {
        TemperatureUnit::Celsius
    }
}

#[cfg(test)]
mod tests {
    use super::{locale_default_unit, TemperatureUnit};

    #[test]
    fn locale_defaults_to_fahrenheit_for_us_liberia_myanmar() {
        assert!(matches!(
            locale_default_unit("en-US"),
            TemperatureUnit::Fahrenheit
        ));
        assert!(matches!(
            locale_default_unit("en_US"),
            TemperatureUnit::Fahrenheit
        ));
        assert!(matches!(
            locale_default_unit("en-LR"),
            TemperatureUnit::Fahrenheit
        ));
        assert!(matches!(
            locale_default_unit("en_LR"),
            TemperatureUnit::Fahrenheit
        ));
        assert!(matches!(
            locale_default_unit("my-MM"),
            TemperatureUnit::Fahrenheit
        ));
        assert!(matches!(
            locale_default_unit("my_MM"),
            TemperatureUnit::Fahrenheit
        ));
    }

    #[test]
    fn locale_defaults_to_celsius_for_other_locales() {
        assert!(matches!(
            locale_default_unit("en-GB"),
            TemperatureUnit::Celsius
        ));
        assert!(matches!(
            locale_default_unit("fr-FR"),
            TemperatureUnit::Celsius
        ));
        assert!(matches!(
            locale_default_unit("ja-JP"),
            TemperatureUnit::Celsius
        ));
    }
}
