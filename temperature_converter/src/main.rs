//Program that converts Celcius to Fahrenheit or Fahrenheit to Celcius
//Takes two user inputs, units of measure and the value

use std::io;


fn fahrenheit_to_celsius(degrees: f32) -> f32 {
    (5.0/9.0) * (degrees - 32.0) 
}

fn celsius_to_fahrenheit(degrees: f32) -> f32 {
    (9.0/5.0 * degrees) + 32.0
}

fn main() {

    let mut exit = String::new();

    loop {

        let mut units = String::new();
        let mut value = String::new();

        println!("UoM that temperature is converted from [C/F]: ");
        io::stdin().read_line(&mut units)
            .expect("Failed to load the line");

        if units.trim() == "C" || units.trim() == "F" {
            println!("Temperature value: ");
            io::stdin().read_line(&mut value)
                .expect("Failed to load the line");

            let value: f32 = match value.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            if units.trim() == "C" {
                let mut result = celsius_to_fahrenheit(value);
                println!("Conversion result: {}", result)
            }
            else {
                let mut result = fahrenheit_to_celsius(value);
                println!("Conversion result: {}", result)
            }
        }
        else {
            println!("Unknown units of measure!");
        }
        
        println!("Press Q to quit, any other key to rerun.");
        io::stdin().read_line(&mut exit)
            .expect("Failed to load the line");
        if exit.trim() == "Q" {
            break
        }
    }
}
