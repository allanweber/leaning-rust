fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");

    //FLOAT
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // BOOLEAN
    let t = true;

    let f: bool = false; // with explicit type annotation

    //CHAR

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    // Compound types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{} {} {}", tup.0, tup.1, tup.2);
    let (x, y, z) = tup;
    println!("{} {} {}", x, y, z);

    // Arrays
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("{}", months[3]);

    // You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array, like so:
    let a: [i32; 5] = [1, 2, 3, 4, 5];
}
