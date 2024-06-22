fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // shadowing
    let spaces = "   ";
    let spaces = spaces.len(); // can change var type
    println!("The number of spaces is {spaces}.");
    
    // mut
    let mut x = 5;
    x = x + 1; // var type should stay same

    // division
    let truncated = -5.0 / 3.0;
    println!("{truncated}")
}
