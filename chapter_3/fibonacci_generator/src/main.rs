fn main() {
    for i in 1..11 {
        println!("{}", fibonacci(i));
    }
}

fn fibonacci(n:u32)->u32 {
   
    let mut n = n as i32;
    let mut a = 0;
    let mut b = 1;

    while n > 1 {
        let temp = b;
        b = a + b;
        a = temp;
        n-= 1;
    }
    a
}
