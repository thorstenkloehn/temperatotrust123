/// Konvertiert Fahrenheit in Celsius.
/// # Beispiel
/// let celsius = fahrenheit_to_celsius(35);
pub fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}