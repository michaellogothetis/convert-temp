# convert-temp

## Convert temperatures between different temperature scales

### Background

This was a learning exercise. If you need a way to handle temperature and other measurments in Rust, look at the measurements crate (https://docs.rs/measurements/latest/measurements/#).

### Features

A command-line tool and Rust crate for converting temperatures.

The **Temperature** crate defines the following constants:

```
use crate::temperature::{ABSOLUTE_ZERO, BOILING_POINT, FREEZING_POINT};
```

The **Temperature** crate implements the following temperature conversions:

**Celsius to:**

Fahrenheit: degreesF = degreesC * 1.8 + 32.0

kelvin: K = degreesC + 273.15

**Fahrenheit to:**

Celsius: degreesC = (degreesF - 32.0) / 1.8

kelvin: K = (degreesF - 32.0) / 1.8 + 273.15

**kelvin to:**

Celsius: degreesC = K - 273.15

Fahrenheit: degreesF = (K - 273.15) * 1.8 + 32.0

### Example

**Command-Line Usage**

**convert-temp** [-h,--help] [-V,--version] fromTempValue fromTempUnit [toTempUnit]

**Units:** **C**elsius, **F**ahrenheit or **K**elvin

_Example:_

\> **convert_temp** 98.6 F C

98.6°F = 37°C

\> **convert_temp** -273.15 C K

-273.15°C = 0K

**Temperature Crate Usage**

```
use crate::temperature::Temperature;
use crate::temperature::TemperatureUnit;
use crate::temperature::{ABSOLUTE_ZERO, BOILING_POINT, FREEZING_POINT};

let absolute_zero_kelvin = match Temperature::new(ABSOLUTE_ZERO.value, ABSOLUTE_ZERO.unit) {
    Ok(temp) => temp,
    Err(err) => {
        eprintln!("Invalid temperature: {err}");
        return;
    }
};
let absolute_zero_celsius = absolute_zero_kelvin.to(TemperatureUnit::Celsius);

println!("Absolute 0 is {absolute_zero_celsius}.");

let boiling_point_celsius = match Temperature::new(BOILING_POINT.value, BOILING_POINT.unit) {
    Ok(temp) => temp,
    Err(err) => {
        eprintln!("Invalid temperature: {err}");
        return;
    }
};
let boiling_point_fahrenheit = boiling_point_celsius.to(TemperatureUnit::Fahrenheit);

println!("The boiling point of water at sea level is {boiling_point_celsius} or {boiling_point_fahrenheit}.");

let freezing_point_celsius = match Temperature::new(FREEZING_POINT.value, FREEZING_POINT.unit) {
    Ok(temp) => temp,
    Err(err) => {
        eprintln!("Invalid temperature: {err}");
        return;
    }
 };
let freezing_point_fahrenheit = freezing_point_celsius.to(TemperatureUnit::Fahrenheit);
let freezing_point_kelvin = freezing_point_celsius.to(TemperatureUnit::Kelvin);

println!("The temperature at which water freezes can be expressed as {freezing_point_celsius}, {freezing_point_fahrenheit} or {freezing_point_kelvin}.");

```
