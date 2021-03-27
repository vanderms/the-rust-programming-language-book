use std::io;

fn main() {    
    loop {
        run_app();
    }   
}

fn run_app(){
    println!("Enter C to convert from Celsius to Fahrenheit or F to convert from Fahrenheit to Celsius:");
   

    let convert =  loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();
        if input == "C" || input == "F"{
            break input;      
        }
        else  {
            println!("Invalid input. ");
        }
    };

    println!("Enter a temperature");
    let temperature = loop {
        
        let mut input = String::new();
        
        io::stdin().read_line(&mut input).unwrap();
        
        let input: i32 = match input.trim().parse(){
            Ok(value) => value,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };

        break input;
    };
    
    if convert == "C" {
        println!("{} Fanhreit.", (((9.0 / 5.0) * (temperature as f64)) + 32.0));
    }
    else {
        println!("{} Celsius.", ((temperature as f64) - 32.0) / 1.8);
    }
}