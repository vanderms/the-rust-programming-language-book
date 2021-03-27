fn main() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if_expression();
    loop_returning_values();
}

fn if_expression(){
    let condition = true;
    let number = if condition {5} else {12};
    println!("{}", number);
}

//"Rust needs to know at COMPILE time what type the variable is"

fn loop_returning_values(){
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Loop: counter == {}", result);
}