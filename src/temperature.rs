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

#[derive(PartialEq, Eq, Clone, Copy)] 
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

impl TemperatureUnit {
    fn description(&self) -> &str {
        match *self {
            TemperatureUnit::Celsius => "Celsius",
            TemperatureUnit::Fahrenheit => "Fahrenheit",
            TemperatureUnit::Kelvin => "kelvin",
        }
    }

    fn abbreviation(&self) -> &str {
        match *self {
            TemperatureUnit::Celsius => "C",
            TemperatureUnit::Fahrenheit => "F",
            TemperatureUnit::Kelvin => "K",
        }
    }
}

pub type Result<T> = std::result::Result<T, InvalidTemperature>;

#[derive(Debug, Clone)]
pub struct InvalidTemperature;

impl fmt::Display for InvalidTemperature {
    fn fmt(&self, f: &mut fmt:: Formatter) -> fmt::Result {
        write!(f, "Temperature less than 0.0k")
    }
}

pub struct Temperature {
    pub value: f64,
    pub unit: TemperatureUnit,
}

pub const ABSOLUTE_ZERO: Temperature = Temperature {
    value: 0.0,
    unit: TemperatureUnit::Kelvin,
};

pub const BOILING_POINT: Temperature = Temperature {
    value: 100.0,
    unit: TemperatureUnit::Celsius,
};

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
    pub fn new(value: f64, unit: TemperatureUnit) -> Result<Temperature> {
        let temp = Temperature { value, unit };
        if temp.to(TemperatureUnit::Kelvin).value < 0.0 {
            Err(InvalidTemperature)
        }
        else {
            Ok(temp)
        }
    }
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

#[cfg(test)]
mod tests {
    use super::{Temperature, TemperatureUnit, ABSOLUTE_ZERO, BOILING_POINT, FREEZING_POINT};

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
}
