

fn main() {
  
    let sequence = ["first", "second", "third", "fourth",
        "fifth", "sixth", "seventh", "eigth", "nine", "ten", "eleven", "twelve"];
    
    for n in 1..13 {
        println!("On the {} day of Christmas my true love gave to me", sequence[n as usize - 1]);
        print_gifts(n, n);
        println!("\n");
    }
}

fn print_gifts(day:i32, total:i32) {

    if day == 0 {
        return;
    }

    if day != total {
        if day == 1 {
            print!(" and ");
        }
        else {
            print!(", ");
        }
    }

    let gifts = [
        "A partridge in a pear tree", 
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a laying",
        "Seven swans a swimming",
        "Eight maids a milking",
        "Nine ladies dancing",
        "Ten lords a leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    print!("{}", gifts[day as usize - 1]);
    print_gifts( day - 1, total);
}
