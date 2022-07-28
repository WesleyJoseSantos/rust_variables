use std::ops::Index;

fn main() {
    mutt();
    variables();
    special();
    let sum = my_func(33, 55);
    println!("Sum result is {}", sum);

    control_flow();

    handling_loops();
}

fn mutt() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The valie of x is: {}", x);

    const SUB_COUNT: u32 = 100_000;

    println!("The value of SUB_COUNT is: {}", SUB_COUNT)
}

fn variables() {
    // Intergers

    let a = 98_222;
    let b = 0xff;
    let c = 0o77;
    let d = 0b1111_0000;
    let e = b'A';
    let f: u8 = 255;

    // Floating-point numbers
    let f = 2.0;
    let g = 3.0;

    // Operations
    let sum = 5 + 10;
    let diff = 94.5 - 4.3;
    let prod = 4 * 50;
    let quo = 48 / 4;
    let remainder = 5 % 2;

    // Booleans
    let t = true;
    let f: bool = false;

    // Character
    let g = 'a';
    let z = '.';
}

fn special() {
    let tup = ("Let's Get Rusty!", 100_000);
    let (ch, sub_count) = tup;
    let err_codes = [0, 1, 2, 3];
    let not_found = err_codes[1];
    let eight_zeros = [0; 8];

    println!("Tuple [0]: {}", tup.0);
    println!("Tuple:[1]: {}\n", tup.1);

    println!("ch: {}", ch);
    println!("sub_count: {}", sub_count);
}

fn my_func(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    x + y
}

fn control_flow() {
    let condition = false;
    let num = if condition { 2 } else { 22 };

    if num < 10 {
        println!("first condition is true");
    } else if num == 22 {
        println!("second condition is true");
    } else {
        println!("condition was false");
    }
}

fn handling_loops() {
    let mut count = 0;

    let result = loop {
        count += 1;
        println!("again!");

        if count > 10 {
            break count;
        }
    };

    println!("The result is: {}", result);

    while count != 0 {
        println!("{}!", count);
        count -= 1;
    }

    println!("LIFTOFF!!!");

    let collection = [10, 20, 30, 40, 50];

    for element in collection.iter()  {
        println!("value of element: {}", element);
    }

    for number in 1..4 {
        println!("{}", number);
    }

    // this is line comment

    /*
        This is block comment.
    */
}
