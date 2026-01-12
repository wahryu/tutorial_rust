// WHILE LOOP
fn while_loop() {
    let mut count = 0;

    while counter <= 10 {
        if counter % 2 == 0 {
            println!("{}", count);
        }

        count += 1;
    }
}

// WHILE LOOP ARRAY
fn while_array() {
    let arr: [&str; 5] = ["1","2","3","4","5"];
    let mut index = 0;

    while index < arr.len() {
        println!("Value: {}", arr[index]);
        index += 1;
    }
}