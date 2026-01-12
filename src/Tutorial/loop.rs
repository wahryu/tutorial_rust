// LOOP

fn loop_ex() {
    let mut counter = 0;

    loop {
        counter += 1;

        if counter = 10 {
            // break = STOP
            // continue = NEXT or CONTINUE or JUMP
            break;
        }

        println!("Counter : {}", counter);
    }
}

// LOOP with Result

fn loop_ex() {
    let mut counter = 0;
    let result;

    result = 
    loop {
        counter += 1;

        if counter > 10 {
            // break = STOP
            // continue = NEXT or CONTINUE or JUMP
            break counter * 2; // result = counter * 2
        }
    }
    ;

    println!("{}", result);
}

// LOOP outer LOOP
fn outer_loop() {
    let mut num:i32 = 1;

    // First must ' ( tanda petik 1 )
    'outer: loop {
        let mut i = 1;
        loop {
            if number > 10 {
                break 'outer;
            }

            println("{}", number)
        }
    }
}