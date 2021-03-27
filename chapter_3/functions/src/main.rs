fn main() {
    println!("Hello, world!");

    another_function();
    println!("{}", factorial(5));
    block_expressions();
    println!("{}", sum(plus_one(20), 21));
}

fn another_function() {
    println!("Another function.");
}

fn factorial(n:u32)->u32{
    return if n <= 1 {1} else {n * factorial(n - 1)};
}

fn block_expressions(){
    let x = 5;
    let y :i32 = {
        let x = 3;
        x + 1
    };
    println!("x: {}, y: {}", x, y);
}

fn sum(a:i32, b:i32)->i32 {
    a + b
} 

fn plus_one(x: i32)->i32{
    x + 1
}