fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false")
    }

    if number != 0 {
        println!("number was something other than zero");
    }

    if number % 2 == 0 {
        println!("number is divisible by 2");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else {
        println!("number is not divisible by 3, or 2");
    }

    let condition = true;
    let n1 = if condition { 5 } else { 42 };
    println!("n1={n1}");
}
