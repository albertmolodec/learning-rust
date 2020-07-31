use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use std::io;

fn convert_fahrenheit_to_celsius(temp: f32) -> f32 {
    (temp - 32.) * 5. / 9.
}

fn convert_celsius_to_fahrenheit(temp: f32) -> f32 {
    (temp * 9. / 5.) + 32.
}

fn main() {
    let selections = &["Fahrenheit -> Celsius", "Celsius -> Fahrenheit"];
    let temp_units = &["Fahrenheit", "Celsius"];

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Choose direction")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        println!("Please, enter temp in {}", temp_units[selection]);

        let mut temp = String::new();
        io::stdin().read_line(&mut temp).unwrap();
        let temp: f32 = temp.trim().parse().unwrap();

        let result: f32;
        let result_selection;

        if selection == 0 {
            result = convert_fahrenheit_to_celsius(temp);
            result_selection = 1;
        } else {
            result = convert_celsius_to_fahrenheit(temp);
            result_selection = 0;
        }

        println!(
            "Temperature in {} is {}",
            temp_units[result_selection], result
        );

        if !Confirm::new()
            .with_prompt("Another one?")
            .interact()
            .unwrap()
        {
            break;
        }
    }
}
