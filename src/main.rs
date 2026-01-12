fn main() {
    println!("Hello, world!");
    let num: i32 = 10;
    println!("{}", num);
}

#[test]
fn testing() {
    // println!("{}", _if_else());
    // _loop_ex();
    let result: i32 = _factorial(3);
    println!("Result : {}", result);
}

fn _if_else() -> String {
    let num: i32 = 10;
    let result: String;

    result = 
        if num > 5 {
            String::from("{num} > 5")
        } else if num <= 5 {
            String::from("{num} <= 5")
        } else {
            String::from("num")
        }
    ;

    return result;
}

fn _loop_ex() {
    let mut counter: i32 = 0;
    let result: i32;

    result = 
    loop {
        counter += 1;

        if counter > 10 {
            // break = STOP
            // continue = NEXT or CONTINUE or JUMP
            break counter * 2;
        }
    }
    ;

    println!("{}", result);
}

fn _factorial(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result: i32 = 1;
    for i in 1..=n {
        result *= i;
    }

    return result;
}
