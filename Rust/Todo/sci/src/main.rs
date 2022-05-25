fn main() {
    println!("{}", Temperature::Celsius.to_kelvin(32.0));
    println!("{}", Temperature::Kelvin.to_kelvin(32.0));
    println!("{}", Temperature::Fahrenheit.to_kelvin(32.0));
}

pub enum Temperature {
    Celsius,
    Fahrenheit,
    Kelvin,
}
impl Temperature {
    pub fn to_kelvin(&self, n: f32) -> f32 {
        match self {
            Temperature::Kelvin => n,
            Temperature::Celsius => celsius_to_kelvin(n),
            Temperature::Fahrenheit => fahrenheit_to_kelvin(n),
        }
    }
    pub fn to_celsius(&self, n: f32) -> f32 {
        match self {
            Temperature::Kelvin => kelvin_to_celsius(n),
            Temperature::Celsius => n,
            Temperature::Fahrenheit => fahrenheit_to_celsius(n),
        }
    }
    pub fn to_fahrenheit(&self, n: f32) -> f32 {
        match self {
            Temperature::Kelvin => kelvin_to_fahrenheit(n),
            Temperature::Celsius => celsius_to_fahrenheit(n),
            Temperature::Fahrenheit => n,
        }
    }
}
pub fn fahrenheit_to_kelvin(fahrenheit: f32) -> f32 {
    (5.0 / 9.0 * (fahrenheit - 32.0)) + 273.15
}
pub fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    5.0 / 9.0 * (fahrenheit - 32.0)
}

pub fn celsius_to_kelvin(celsius: f32) -> f32 {
    celsius + 273.15
}
pub fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    9.0 / 5.0 * (celsius + 32.0)
}
pub fn kelvin_to_celsius(kelvin: f32) -> f32 {
    kelvin - 273.15
}
pub fn kelvin_to_fahrenheit(kelvin: f32) -> f32 {
    celsius_to_fahrenheit(kelvin_to_celsius(kelvin))
}
