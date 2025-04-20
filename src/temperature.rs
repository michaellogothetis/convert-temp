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
                        let _temperature = self.value * 1.8 + 32.0;
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
                        let _temperature = (self.value * 5.0 / 9.0 ) + 32.0;
                        let _t = Temperature {
                            value: _temperature,
                            unit: TemperatureUnit::Fahrenheit,
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