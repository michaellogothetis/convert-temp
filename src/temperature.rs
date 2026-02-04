//! Temperature conversion types and helpers.
//!
//! Provides a `Temperature` struct, conversion between `C`, `F`, and `K`,
//! common constants, parsing from strings, and display formatting.
//!
//! # Examples
//! ```rust
//! use convert_temp::temperature::{Temperature, TemperatureUnit};
//!
//! let t = Temperature::new(100.0, TemperatureUnit::Celsius).unwrap();
//! let f = t.to(TemperatureUnit::Fahrenheit);
//! assert_eq!(format!("{f}"), "212\u{00B0}F");
//! ```
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
//   Fahrenheit - degreesF = degreesC * 1.8 + 32.0
//   kelvin - degreesK = degreesC + 273.15
//
// Fahrenheit to:
//   Celsius - degreesC = (degreesF - 32.0) / 1.8
//   kelvin - degreesK = (degreesF - 32.0) / 1.8 + 273.15
//
// kelvin to:
//   Celsius - degreesC = degreesK - 273.15
//   Fahrenheit - degreesF = (degreesK - 273.15) * 1.8 + 32.0

// temperature = Temp(temperature, units = "C")
// temperature_fahrenheit = temperature.to_fahrenheit()
// temperature_kelvin = temperature.to_kelvin()
// temperature_celsius = temperature_kelvin.to_celsius()
//
// temp_celsius = to_celsius(temp_fahrenheit)
// temp_fahrenheit = to_fahrenheit(temp_celsius)
// converted_temp = convert_temp(temp) temp: 37.5C or 99.5F

use std::fmt;
use std::str::FromStr;

/// Temperature units.
#[derive(Debug, PartialEq, Eq, Clone, Copy)] 
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl TemperatureUnit {
    /// Returns the long-form name of the unit.
    #[allow(dead_code)]
    fn description(&self) -> &str {
        match *self {
            TemperatureUnit::Celsius => "Celsius",
            TemperatureUnit::Fahrenheit => "Fahrenheit",
            TemperatureUnit::Kelvin => "kelvin",
        }
    }

    /// Returns the one-letter abbreviation for the unit.
    fn abbreviation(&self) -> &str {
        match *self {
            TemperatureUnit::Celsius => "C",
            TemperatureUnit::Fahrenheit => "F",
            TemperatureUnit::Kelvin => "K",
        }
    }
}

/// Result alias for temperature creation failures.
pub type Result<T> = std::result::Result<T, InvalidTemperature>;

/// Error returned when a temperature is below absolute zero.
#[derive(Debug, Clone)]
pub struct InvalidTemperature;

impl fmt::Display for InvalidTemperature {
    fn fmt(&self, f: &mut fmt:: Formatter) -> fmt::Result {
        write!(f, "Temperature less than 0.0k")
    }
}

/// Errors that can occur when parsing a temperature from a string.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemperatureParseError {
    Empty,
    MissingUnit,
    InvalidUnit(char),
    InvalidNumber,
    BelowAbsoluteZero,
}

impl fmt::Display for TemperatureParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemperatureParseError::Empty => write!(f, "Empty temperature string"),
            TemperatureParseError::MissingUnit => write!(f, "Missing temperature unit"),
            TemperatureParseError::InvalidUnit(unit) => {
                write!(f, "Invalid temperature unit '{unit}'")
            }
            TemperatureParseError::InvalidNumber => write!(f, "Invalid temperature number"),
            TemperatureParseError::BelowAbsoluteZero => {
                write!(f, "Temperature less than 0.0k")
            }
        }
    }
}

/// A temperature value paired with its unit.
#[derive(Debug, Clone, Copy)]
pub struct Temperature {
    pub value: f64,
    pub unit: TemperatureUnit,
}

/// Absolute zero in Kelvin.
pub const ABSOLUTE_ZERO: Temperature = Temperature {
    value: 0.0,
    unit: TemperatureUnit::Kelvin,
};

/// Boiling point of water at sea level in Celsius.
pub const BOILING_POINT: Temperature = Temperature {
    value: 100.0,
    unit: TemperatureUnit::Celsius,
};

/// Freezing point of water in Celsius.
pub const FREEZING_POINT: Temperature = Temperature {
    value: 0.0,
    unit: TemperatureUnit::Celsius,
};

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.unit {
            TemperatureUnit::Kelvin => {
                write!(f, "{}{}", self.value, self.unit.abbreviation())
            },
            _other => {
                write!(f, "{}\u{00B0}{}", self.value, self.unit.abbreviation())
            }

        }
        
    }
}

impl Temperature {
    /// Creates a new temperature, rejecting values below absolute zero.
    pub fn new(value: f64, unit: TemperatureUnit) -> Result<Temperature> {
        let temp = Temperature { value, unit };
        if temp.to(TemperatureUnit::Kelvin).value < 0.0 {
            Err(InvalidTemperature)
        }
        else {
            Ok(temp)
        }
    }

    /// Parses a temperature from a string like `\"37.5C\"`, `\"32F\"`, or `\"273.15K\"`.
    pub fn from_str(input: &str) -> std::result::Result<Temperature, TemperatureParseError> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(TemperatureParseError::Empty);
        }

        if trimmed.len() < 2 {
            return Err(TemperatureParseError::MissingUnit);
        }

        let (value_part, unit_part) = trimmed.split_at(trimmed.len() - 1);
        let unit_char = unit_part
            .chars()
            .next()
            .ok_or(TemperatureParseError::MissingUnit)?;

        let unit = match unit_char {
            'C' => TemperatureUnit::Celsius,
            'F' => TemperatureUnit::Fahrenheit,
            'K' => TemperatureUnit::Kelvin,
            _ => return Err(TemperatureParseError::InvalidUnit(unit_char)),
        };

        let value_str = value_part.trim();
        if value_str.is_empty() {
            return Err(TemperatureParseError::InvalidNumber);
        }

        let value: f64 = value_str
            .parse()
            .map_err(|_| TemperatureParseError::InvalidNumber)?;

        Temperature::new(value, unit).map_err(|_| TemperatureParseError::BelowAbsoluteZero)
    }
    /// Converts this temperature to the requested unit.
    pub fn to(&self, unit: TemperatureUnit) -> Temperature {
        match self.unit {
            TemperatureUnit::Celsius => {
                match unit {
                    TemperatureUnit::Celsius =>
                        Temperature {
                            value: self.value,
                            unit: self.unit,
                        },
                    TemperatureUnit::Fahrenheit => {
                        let _temperature = self.value * 9.0 / 5.0 + 32.0;
                        let _t = Temperature {
                            value: _temperature,
                            unit: TemperatureUnit::Fahrenheit,
                        };
                        _t
                    },
                    TemperatureUnit::Kelvin => {
                        let _temperature = self.value + 273.15;
                        let _t = Temperature {
                            value: _temperature,
                            unit: TemperatureUnit::Kelvin,
                        };
                        _t
                    }
                }
            },
            TemperatureUnit::Fahrenheit => {
                match unit {
                    TemperatureUnit::Celsius => {
                        let _temperature = (self.value - 32.0 ) * 5.0 /9.0;
                        let _t = Temperature {
                            value: _temperature,
                            unit: TemperatureUnit::Celsius,
                        };
                        _t
                    },
                    TemperatureUnit::Fahrenheit =>
                        Temperature {
                            value: self.value,
                            unit: self.unit,
                        },
                    TemperatureUnit::Kelvin =>
                        self.to(TemperatureUnit::Celsius).to(TemperatureUnit::Kelvin)
                }
            },
            TemperatureUnit::Kelvin => {
                match unit {
                    TemperatureUnit::Celsius => {
                        let _temperature = self.value - 273.15;
                        let _t = Temperature {
                            value: _temperature,
                            unit: TemperatureUnit::Celsius,
                        };
                        _t
                    },
                    TemperatureUnit::Fahrenheit =>
                        self.to(TemperatureUnit::Celsius).to(TemperatureUnit::Fahrenheit),
                    TemperatureUnit::Kelvin => {
                        Temperature {
                            value: self.value,
                            unit: self.unit,
                        }
                    }
                
                }
            }
        }
    }
}

/// Parses a temperature using the standard `FromStr` trait.
impl FromStr for Temperature {
    type Err = TemperatureParseError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Temperature::from_str(s)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Temperature,
        TemperatureParseError,
        TemperatureUnit,
        ABSOLUTE_ZERO,
        BOILING_POINT,
        FREEZING_POINT,
    };

    fn assert_close(actual: f64, expected: f64, epsilon: f64) {
        assert!(
            (actual - expected).abs() <= epsilon,
            "expected {expected}, got {actual} (epsilon {epsilon})"
        );
    }

    #[test]
    fn celsius_to_fahrenheit_basic() {
        let temp_c = Temperature::new(100.0, TemperatureUnit::Celsius).expect("valid");
        let temp_f = temp_c.to(TemperatureUnit::Fahrenheit);
        assert_close(temp_f.value, 212.0, 1e-9);
        assert!(matches!(temp_f.unit, TemperatureUnit::Fahrenheit));
    }

    #[test]
    fn fahrenheit_to_celsius_basic() {
        let temp_f = Temperature::new(32.0, TemperatureUnit::Fahrenheit).expect("valid");
        let temp_c = temp_f.to(TemperatureUnit::Celsius);
        assert_close(temp_c.value, 0.0, 1e-9);
        assert!(matches!(temp_c.unit, TemperatureUnit::Celsius));
    }

    #[test]
    fn celsius_to_kelvin_basic() {
        let temp_c = Temperature::new(0.0, TemperatureUnit::Celsius).expect("valid");
        let temp_k = temp_c.to(TemperatureUnit::Kelvin);
        assert_close(temp_k.value, 273.15, 1e-9);
        assert!(matches!(temp_k.unit, TemperatureUnit::Kelvin));
    }

    #[test]
    fn kelvin_to_celsius_basic() {
        let temp_k = Temperature::new(273.15, TemperatureUnit::Kelvin).expect("valid");
        let temp_c = temp_k.to(TemperatureUnit::Celsius);
        assert_close(temp_c.value, 0.0, 1e-9);
        assert!(matches!(temp_c.unit, TemperatureUnit::Celsius));
    }

    #[test]
    fn absolute_zero_celsius_is_valid() {
        let temp_c = Temperature::new(-273.15, TemperatureUnit::Celsius).expect("valid");
        let temp_k = temp_c.to(TemperatureUnit::Kelvin);
        assert_close(temp_k.value, 0.0, 1e-9);
    }

    #[test]
    fn below_absolute_zero_rejected() {
        let temp_c = Temperature::new(-273.151, TemperatureUnit::Celsius);
        assert!(temp_c.is_err());
    }

    #[test]
    fn absolute_zero_fahrenheit_is_valid() {
        let temp_f = Temperature::new(-459.67, TemperatureUnit::Fahrenheit).expect("valid");
        let temp_k = temp_f.to(TemperatureUnit::Kelvin);
        assert_close(temp_k.value, 0.0, 1e-2);
    }

    #[test]
    fn display_formats_celsius_with_degree_symbol() {
        let temp_c = Temperature::new(25.0, TemperatureUnit::Celsius).expect("valid");
        assert_eq!(format!("{temp_c}"), "25\u{00B0}C");
    }

    #[test]
    fn display_formats_fahrenheit_with_degree_symbol() {
        let temp_f = Temperature::new(77.0, TemperatureUnit::Fahrenheit).expect("valid");
        assert_eq!(format!("{temp_f}"), "77\u{00B0}F");
    }

    #[test]
    fn display_formats_kelvin_without_degree_symbol() {
        let temp_k = Temperature::new(300.0, TemperatureUnit::Kelvin).expect("valid");
        assert_eq!(format!("{temp_k}"), "300K");
    }

    #[test]
    fn constants_match_expected_values() {
        assert_close(ABSOLUTE_ZERO.value, 0.0, 1e-12);
        assert!(matches!(ABSOLUTE_ZERO.unit, TemperatureUnit::Kelvin));

        assert_close(BOILING_POINT.value, 100.0, 1e-12);
        assert!(matches!(BOILING_POINT.unit, TemperatureUnit::Celsius));

        assert_close(FREEZING_POINT.value, 0.0, 1e-12);
        assert!(matches!(FREEZING_POINT.unit, TemperatureUnit::Celsius));
    }

    #[test]
    fn parse_temperature_from_string() {
        let temp = Temperature::from_str("37.5C").expect("valid");
        assert_close(temp.value, 37.5, 1e-12);
        assert!(matches!(temp.unit, TemperatureUnit::Celsius));

        let temp = Temperature::from_str("32F").expect("valid");
        assert_close(temp.value, 32.0, 1e-12);
        assert!(matches!(temp.unit, TemperatureUnit::Fahrenheit));

        let temp = Temperature::from_str(" 273.15K ").expect("valid");
        assert_close(temp.value, 273.15, 1e-12);
        assert!(matches!(temp.unit, TemperatureUnit::Kelvin));
    }

    #[test]
    fn parse_temperature_rejects_invalid_unit() {
        let err = Temperature::from_str("10X").unwrap_err();
        assert!(matches!(err, TemperatureParseError::InvalidUnit('X')));
    }

    #[test]
    fn parse_temperature_rejects_invalid_number() {
        let err = Temperature::from_str("abcC").unwrap_err();
        assert!(matches!(err, TemperatureParseError::InvalidNumber));
    }

    #[test]
    fn parse_temperature_rejects_below_absolute_zero() {
        let err = Temperature::from_str("-300C").unwrap_err();
        assert!(matches!(err, TemperatureParseError::BelowAbsoluteZero));
    }

    #[test]
    fn parse_temperature_via_fromstr_trait() {
        let temp: Temperature = "451F".parse().expect("valid");
        assert_close(temp.value, 451.0, 1e-12);
        assert!(matches!(temp.unit, TemperatureUnit::Fahrenheit));
    }
}
