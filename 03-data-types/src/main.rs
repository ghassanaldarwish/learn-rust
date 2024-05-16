// data types in Rust

fn main() {
    // Data types:
    // Scalar types
    // 1. Integer   Signed unsigned
    // - 8-bit      i8	   u8
    let small_number: i8 = -127;
    println!("The value of small_number is: {}", small_number);
    let small_number_2: u8 = 127;
    println!("The value of small_number_2 is: {}", small_number_2);

    // - 16-bit 	i16	   u16
    // - 32-bit 	i32	   u32
    // - 64-bit 	i64	   u64
    // - 128-bit 	i128   u128
    let big_number: i128 = -123456789012345678901234567890;
    println!("The value of big_number is: {}", big_number);
    let big_number_2: u128 = 123456789012345678901234567890;
    println!("The value of big_number_2 is: {}", big_number_2);
    // - arch 	    isize  usize

    // 2. Floating-point
    // - f32 
    let float_number: f32 = 3.14;
    println!("The value of float_number is: {}", float_number);
    // - f64
    let double_number: f64 = 3.14159265358979323846264338327950288419716939937510582097494459230781640628620899862803482534211706798214808651328230664709384460955058223172535940812848111745028410270193;
    println!("The value of double_number is: {}", double_number);

    // 3. Boolean
    let is_true: bool = false;

    if is_true {
        println!("The value of is_true is: {}", is_true);
    } else {
        println!("The value of is_true is: {}", is_true);
    }

    let not_bool: bool = !is_true;
    println!("The value of not_bool is: {}", not_bool);

    // 4. Character
    let a: char = 'A';
    let character: char = '😻';
    let b: char = 'X';
    println!("The values are: {} {} {}", a, character, b); 


    // 5. Tuples
    let tup: (i32, f64, char) = (42, 3.14, 'A'); 
    println!("The values are: {} {} {}", tup.0, tup.1, tup.2);


    // Destructuring
    let (x, y, z) = tup;

    println!("The values are: {} {} {}", x, y, z);  


    // 6. Arrays (with fixed length) - all elements must be of the same type
    let arr = [1, 2, 3, 4, 5];   

    println!("The values are: {} {} {}", arr[0], arr[1], arr[2]);   


    for element in arr.iter() {
        println!("The value is: {}", element);
    }                                     

// Custom data types
    // 1. Structs (good to have group of related values) - similar to interface and objects in JS
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let user1 = User {
        username: String::from("ghassan"),
        email: String::from("test@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("The values are: {} {} {} {}", user1.username, user1.email, user1.sign_in_count, user1.active);    
    
    // 2. Enums (good to have group of related values) - similar to enums in JS
 
    enum Color {
        Red,
        Green,
        Blue,
  
    }

    //let color = Color::Red;
   // let color = Color::Green;
    let color = Color::Blue;


    // similar to switch case in JS
    match color {
        Color::Red => println!("The color is Red"),
        Color::Green => println!("The color is Green"),
        Color::Blue => println!("The color is Blue"),
    }




}
