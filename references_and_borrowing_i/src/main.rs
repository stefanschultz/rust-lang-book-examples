fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    // change(&mut s);

    /*
    let r1 = &mut s;
    let r2 = &mut s; // only one reference allowed

    println!("{r1}, {r2}");
    */

    // let reference_to_nothing = dangle();

    let s = no_dangle();
    println!("{s}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// function will not work!
/* fn change(some_string: &String) { // immutable reference
    some_string.push_str(", world");
} */

// will work
fn change(some_string: &mut String) {
    // mutable reference
    some_string.push_str(", world");
}

/*fn dangle() -> &String {
    let s = String::from("hello");

    &s
}*/

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}