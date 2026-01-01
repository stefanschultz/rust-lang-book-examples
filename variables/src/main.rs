const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    println!("constant: {}", THREE_HOURS_IN_SECONDS);

    // shawdowing I
    let y: i32 = 5;

    let y: i32 = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");

    // shawdowing II
    let spaces = "    ";
    let spaces = spaces.len();

    // not allowed to use mut, while different data types used
    // let mut spaces = "    ";
    // spaces = spaces.len();

    println!("len of spaces: {}", spaces);

    let g: u32 = "42".parse().expect("Not a number!");
    println!("parsed val: {}", g);

    // tuple
    let tup = (42, "S", 6.4, true);
    println!("tuple: {:#?}", tup);
    let (a, b, c, d) = tup; // destructing data (unpacking)
    let e = tup.0;
    println!("{a}, {b}, {c}, {d} {e}");

    // array
    let arr = [1,2,3,4,5];
    println!("{:#?}", arr);

    let _arr2: [i32; 3] = [6,7,8];
    let _arr3 = [3; 5]; // = [3,3,3,3,3];

}
