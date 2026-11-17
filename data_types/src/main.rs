// Floating-point types
// fn main() {
//     let x = 2.0; // f64
//     println!("The value of x is: {x:?}");

//     let y: f32 = 3.0; // f32
//     println!("The value of y is: {y:.20}");
// }

// Numerical Operations
// fn main() {
//     // addition
//     let _sum = 5 + 10;

//     // subtraction
//     let _difference = 95.5 - 4.3;

//     // multiplication
//     let _product = 4 * 30;

//     // division
//     let _quotient = 56.7 / 32.2;
//     println!("The value of quotient is: {_quotient}");
//     let _truncated = -5 / 3; // Results in -1
//     println!("The value of truncated is: {_truncated}");

//     // remainder
//     let _remainder = 43 % 5;
// }

// // Boolean type
// fn main() {
//     let _t = true;

//     let _f: bool = false; // with explicit type annotation
// }

// // The character type
// fn main() {
//     let _c = 'z';
//     let _z: char = 'ℤ'; // with explicit type annotation
//     let _heart_eyed_cat = '😻';
// }

/*  Compound Types */
// // Tuple type
// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0;
//     let six_point_four = x.1;
//     let one = x.2;

//     println!(
//         "The value of x is:\n\
//          five_hundred: {five_hundred},\n\
//          six_point_four: {six_point_four},\n\
//          one: {one}"
//     );
// }

// // Unit typle type
// fn main() {
//     let unit: () = (); // the empty tuple
//     println!("unit is: {unit:?}");

//     let result = do_nothing();
//     println!("do_nothing returned: {result:?}");
//     assert_eq!(result, unit);

//     // A block ending in a statement (semicolon) evaluates to ()
//     let block = {
//         let _a = 1 + 2;
//     };
//     println!("block is: {block:?}");
// }
// // No return type is declared, so this implicitly returns unit tuple => ()
// fn do_nothing() {}

// // Array Type
// fn main() {
//     let a = [1, 2, 3, 4, 5];
//     let months = [
//         "January",
//         "February",
//         "March",
//         "April",
//         "May",
//         "June",
//         "July",
//         "August",
//         "September",
//         "October",
//         "November",
//         "December",
//     ];

//     println!("The value of a is: {a:?}");
//     println!("The value of months is: {months:?}");

//     //  type of each element, a semicolon, and then the number of elements
//     let _a: [i32; 5] = [1, 2, 3, 4, 5];

//     // You can also initialize an array to contain the same value for each element
//     let _a = [3; 5]; // same as let a = [3, 3, 3, 3, 3];

//     // Access selements
//     let a = [1, 2, 3, 4, 5];

//     let first = a[0];
//     let second = a[1];

//     println!("The value of first is: {first}");
//     println!("The value of second is: {second}");
// }

// Invalid Array Element Access

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
