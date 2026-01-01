fn main() {
    let s = String::from("hello");

    takes_ownershio(s);

    // print!("{s}"); // borrow of moved value: `s`

    let x = 5;

    makes_copy(x);
}

fn takes_ownershio(some_string: String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}
