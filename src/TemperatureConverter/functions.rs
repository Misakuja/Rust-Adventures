pub(crate) struct Converter {

}

impl Converter {
    pub(crate) fn fahrenheit_to_celsius(fahrenheit_value: f64) -> f64 {
        return (fahrenheit_value - 32.0) * 5.0/9.0;
    }
    pub(crate) fn celsius_to_fahrenheit(celsius_value: f64) -> f64 {
        return (celsius_value * 9.0/5.0) + 32.0;
    }
    pub(crate) fn celsius_to_kelvin(celsius_value: f64) -> f64 {
        return celsius_value + 273.15;
    }
    pub(crate) fn kelvin_to_celsius(kelvin_value: f64) -> f64 {
        return kelvin_value - 273.15;
    }
    pub(crate) fn fahrenheit_to_kelvin(fahrenheit_value: f64) -> f64 {
        return (fahrenheit_value - 32.0) * 5.0/9.0 + 273.15;
    }
    pub(crate) fn kelvin_to_fahrenheit(kelvin_value: f64) -> f64 {
        return 1.8 *(kelvin_value - 273.0) + 32.0;
    }
}