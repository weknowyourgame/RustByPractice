fn main() {
    let x = 5;                              // by default the variables are immutable 
    println!("The value of x is: {}", x);

    let mut y = 5;                          // to make them mutable use mut
    println!("The value of y is: {}", y);

    y = 6;
    println!("The value of y is: {}", y);

    let z = 5;                              // types can be infered
    println!("The value of y is: {}", z);

    let a: i32 = 5;                         // or can be explicitly declared
    println!("The value of a is: {}", a);

    let a = "hello, world";                 // variables can be shadowed, redeclared with a different type, and the value is reassigned 
    println!("The value of a is: {}", a);
}
