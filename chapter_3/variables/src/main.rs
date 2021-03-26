
const MAX_POINTS: u32 = 100_000;

fn main() {
   mutability();
   shadowing();
   tuples();
   arrays();
}

fn mutability(){
    println!("\n1. MUTABILITY");
    let x = 5;
    println!("The value of x is: {}", x);  
    println!("The value of MAX POINTS is: {}", MAX_POINTS);

}


fn shadowing(){
    println!("\n2. SHADOWING");
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("x: {}", x);

    let spaces = " ";
    let spaces = spaces.len();
    println!("spaces length: {}", spaces);
}

//data types: scalar types: integer, float, boolean and char
//compound types: primitive: tuples and arrays
//rust char is four bites 

fn tuples(){
    println!("\n2. TUPLES");
    
    let tup = (500, 6.4, 1);
    println!("{}, {}, {}", tup.0, tup.1, tup.2);

    //destructuring. This programming language is simply awesome!
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);
}

//"Unlike a tuple, every element of an array must have the same type"
fn arrays(){
    println!("\n2. ARRAYS");

    const SIZE :usize = 12;
    let x = 1;
    let a = [x; SIZE];
    for n in a.iter() {
        println!("{}", n);
    }    
}