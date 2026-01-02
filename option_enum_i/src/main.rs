// concept of Option Enum
/*
enum Option<T> {
    None,
    Some(T),
}
*/

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska, // --snip--
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    let some_number = Some(5);
    let some_char = Some('s');
    let absent_number: Option<i32> = None;

    value_in_cents(Coin::Quarter((UsState::Alaska)));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let dice_roll = 4 + 2;
    match dice_roll {
        40 => println!("40"),
        42 => println!("42"),
        _ => (), // return unit type
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
