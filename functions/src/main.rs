fn main() {
    print_labeld_measurement(42, 's');

    // statements
    let _y = 6;

    // expressions
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {y}");

    // function with return value
    let x = five();
    println!("The value of x is: {x}");

    println!("Returned values: {}", plus_one(5));
}

fn print_labeld_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5 // expression return a value
}

fn plus_one(x: i32) -> i32 {
    x + 1 // expression return addition
    // () -> is the `unit type` as return data type
}
