
use std::io;

fn main() {
    println!("*** The Thermometer ***");
    println!("Convert from Celsius to Fahrenheit and vice versa!");

    loop {
        println!("Choose conversion\n1) Celsius to Farenheit.\n2) Farenheit to Celsius.\n3) Quit.");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)
            .expect("Error reading user input");
        let choice:u8  = choice.trim().parse()
            .expect("Please choose between 1 or 2");

        if choice == 3 {
            println!("Good Bye!");
            break;
        }

        println!("Please enter value to convert:");
        let mut value = String::new();
        io:: stdin().read_line(&mut value)
            .expect("Error reading user input");
        let value:f64 = value.trim().parse()
            .expect("Please enter a valid integer");

        if choice == 1 {
            let result = celsius_to_farenheit(value);
            println!("{} degree celsius to farenheit is {} degree farenheit", value, result);
            println!("\n\n");
        } else if choice == 2 {
            let result = farenheit_to_celsius(value);
            println!("{} degree farenheit to celsius is {} degree celsius", value, result);
            println!("\n\n");
        }  else {
            println!("Invalid conversion choice was selected. Try again!");
        }
    }
    
}


fn celsius_to_farenheit(val: f64) -> f64{
    (val * 9.0/5.0) + 32.0
    }

fn farenheit_to_celsius(val: f64) -> f64{
    5.0/9.0 * (val - 32.0)
}
