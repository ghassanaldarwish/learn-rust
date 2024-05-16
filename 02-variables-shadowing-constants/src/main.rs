fn main() {
    let x = 5;
    println!("The value of x is: {}", x);

    // here we get error because we can't reassign a value to a constant
    //x = 6;
    //println!("The value of x is: {}", x);

    // we can declare the variable with the same name

    let x = 4;
    println!("The value of x is: {}", x);

    let mut y= 8;

    println!("The value of y is: {}", y);

    y = 12;
    println!("The value of y is: {}", y);

    {
        // shadowing the variable in inner scope
        let z = 10;
        println!("The value of z is: {}", z);
    }

    // the mutable variable can not change the type

    //y = "Hello";


    // if we use constant we need to specify the type
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}
