fn main() {
    another_function(123);

    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y in a expression is: {y}");

    let five = five();
    println!("the value of five is {five}");

    let plusOne = plus_one(5);
    println!("5 + 1 is {plusOne}");
}

fn another_function(x: i32) {
    println!("the value of x is: {x}");
}

// function with return value
// return functions don't need return keyword or semicolon
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
