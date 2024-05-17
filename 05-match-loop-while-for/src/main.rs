
fn check_num (number: i32) {
    if number % 2 == 0 {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);

        if number > 10 {
            println!("{} is greater than 10", number);
        } else {
            println!("{} is less than or equal to 10", number);
        }
    }
}

// inform the Rust compiler to suppress the dead_code warnings for the specified scope
#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents (number: Coin) -> u8 {
    match number {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
    
}

fn looper () {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Result is: {}", result);
}

fn while_looper () {
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
        // wait for 1 second
      //  std::thread::sleep(std::time::Duration::from_secs(1));
    }
    println!("LIFTOFF!!!");
}

fn for_looper () {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("The value is: {}", element);
    }

    let text = "Hello, World!";

    for c in text.chars() {
        println!("{}", c);
    }

    for num in 1..4 {
        println!("strange {}", num);
    }
}

fn fizz_buzz (number: i32) {
    if number % 3 == 0 && number % 5 == 0 {
        println!("FizzBuzz");
    } else if number % 3 == 0 {
        println!("Fizz");
    } else if number % 5 == 0 {
        println!("Buzz");
    } else {
        println!("ff {}", number);
    }
}




fn main() {
    let number = 15;
    let b = 10;
    fizz_buzz(number);
    check_num(number);
    looper();
    for_looper();
    while_looper() ;
    let coin = Coin::Nickel;
    let value = value_in_cents(coin);
    println!("Value of coin is: {}", value);

    if number > b && number < 20 {
        println!("{} is greater than 10 and less than 20", number);
    } else {
        println!("{} is less than 10 or greater than 20", number);
    }

    if number < b || number > 20 {
        println!("{} is less than 10 or greater than 20", number);
    } else {
        println!("{} is greater than 10 and less than 20", number);
    }

 
}
