fn func_param(first_name: &str, last_name: &str) {
    println!("Full Name : {} {}", first_name, last_name);
}

fn run_func() {
    func_param("W", "U");
}

// Return in function

fn factorial(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result: i32 = 1;
    for i in 1..=n {
        result *= i;
    }

    return result;
}

fn run_factoria_func() {
    let result: i32 = factorial(5);
    println!("Result : {}", result);
}