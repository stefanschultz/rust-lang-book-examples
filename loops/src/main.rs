fn main() {
    // example 1
    loop {
        println!("Hello, world again!");
        break;
    }

    // example 2 loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result of counter is {result}");

    // example 3 loop with marker
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // example 4 while
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // example 5 for
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("(1) the value is: {}", a[index]);
        index += 1;
    }

    // example 6 for
    for element in a {
        println!("(2) the value is: {element}");
    }

    // eaxample 7 for with rev
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF AGAIN!!!");
}
