fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    // spacing
    println!("");

    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope: {y}");
    }

    println!("The value of y is: {y}");

    // interesting usage of shadowing
    let spaces = "    ";
    let spaces = spaces.len();
}
