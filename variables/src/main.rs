fn main() {
    // by using the let keyword on the same variable, we can shadow.
    // This allows us to "change" immutables, and keep the same variable name.
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the iner scope is: {x}");
        // x is 12 here
    }

    println!("The value of x is: {x}");
    // x is 6 here

    let spaces = "   ";
    let spaces = spaces.len();
    println!("There are {} spaces", spaces);
}
