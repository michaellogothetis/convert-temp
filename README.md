[![convert-temp CI](https://github.com/michaellogothetis/convert-temp/actions/workflows/rust.yml/badge.svg)](https://github.com/michaellogothetis/convert-temp/actions/workflows/rust.yml)

# convert-temp

## Convert temperatures between different temperature scales

### Background

This was a learning exercise. If you need a way to handle temperature and other measurments in Rust, look at the measurements crate (https://docs.rs/measurements/latest/measurements/#).

### Features

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

```
temperature = Temp(temperature, units = "C")
temperature_fahrenheit = temperature.to_fahrenheit()
temperature_kelvin = temperature.to_kelvin()
temperature_celsius = temperature_kelvin.to_celsius()

temp_celsius = to_celsius(temp_fahrenheit)
temp_fahrenheit = to_fahrenheit(temp_celsius)
converted_temp = convert_temp(temp)

> temp: 37.5C or 99.5F
```
