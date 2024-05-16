// &str is a string slice, it is a reference to a string
// char is a single character
fn print_number(x: i32 , name: &str , lastname: char) {
  println!("x is: {}", x);
    println!("name is: {}", name);

    println!("lastname is: {}", lastname);

}

// Function return a value

fn add(x: i32, y: i32) -> i32 {
    x + y
}

// Function return multiple values

fn swap(x: i32, y: i32) -> (i32, i32) {
    // if we want stop the function and return the value we can use return keyword
    (y + 4, x )

    // return (y + 4, x )

     // any code after return keyword will not be executed
}

fn main() {
    print_number(13, "John", 'D');
    print_number(10, "Doe", 'J');

    let a = 10;
    let b = 20;
    let result = add(a, b);
    println!("{} + {} = {}", a, b, result);

    let (c, d) = swap(a, b);
    println!("a = {}, b = {}", c, d);

    // OR 
    let res = swap(a, b);
    println!("a = {}, b = {}", res.0, res.1);
   
  // {:?} is used to print the tuple
    println!("a = {}, b = {:?}", res.0, res.1);

    
}
