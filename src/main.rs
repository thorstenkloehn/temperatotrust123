mod library;
use library::{celsius_to_fahrenheit,fahrenheit_to_celsius};
use std::io::stdin;
fn main() {
let mut text =String::new();
    println!("{}", celsius_to_fahrenheit(12.56));
    println!("{}",fahrenheit_to_celsius(54.00));
    stdin().read_line(&mut text).expect("Falsch");
    let wert:f64 = text.trim().parse().expect("Falsch Angabe");
    println!("{}", celsius_to_fahrenheit(wert));
    
}

