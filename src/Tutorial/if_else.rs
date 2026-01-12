fn if_else() {
    let num: i32 = 10;

    if num > 5 {
        println!("{} > 5", num)
    } else if num <= 5 {
        println!("{} <= 5", num)
    } else {
        println!("{}", num)
    }
}

// OR

fn if_else() {
    let num: i32 = 10;
    let result: i32;

    result = 
        if num > 5 {
            println!("{} > 5", num)
        } else if num <= 5 {
            println!("{} <= 5", num)
        } else {
            println!("{}", num)
        }
    ;
    return result;
}