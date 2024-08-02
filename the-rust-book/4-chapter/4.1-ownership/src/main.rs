fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}"); // This will print `hello, world!`

    {
        let scope_s = String::from("hello");
        println!("{scope_s}");
    }

    // This will not work because `scope_s` is out of scope
    // println!("{scope_s}");

    //
    //
    //Double Free error
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s2}, world!");
    //the line bellow will not work because s1 is moved to s2
    //println!("{s1}, world!");

    //
    //
    //Clone is a deep copy create two pointers with two heap memory
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {s1}, s2 = {s2}");

    let x = 5;
    let y = x;

    println!("x = {x}, y = {y}");

    //
    //
    //Ownership and Functions
    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function...
                        // The line bellow will not work because s is moved to the function
                        // println!("{s}");

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function, but i32 is Copy, so it's okay to still use x afterward
                   // The line bellow will work because i32 is Copy
    println!("{x}");

    //
    //
    //Return Values and Scope
    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
}

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it
    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
