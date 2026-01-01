fn main() {
    // example 1
    let mut s = String::from("hello");
    s.push_str(", world"); // // push_str() appends a literal to a String
    println!("{s}"); // prints `hello, world!`

    // example 2
    /*let s1 = String::from("hello");
    let s2 = s1;
    println!("{s1}, world");*/

    // example 3
    let mut s = String::from("hello");
    s = String::from("ahoy");
    println!("{s}, world");

    // example 4 - clone heap data
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

    // example 5
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");
}
