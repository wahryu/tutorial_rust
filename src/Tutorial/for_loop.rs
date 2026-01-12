// FOR LOOP
fn for_loop() {
    let arr: [char; 5] = ['A', 'B', 'C', 'D', 'E'];

    for value in arr {
        println!("Value : {}", value)
    }
}

// FOR in RANGE (mirip Python)

fn for_in_range() {
    let arr: [&str; 3] = ["A", "1", "a"];

    let range = 0..5; // di Python range(0,5) 
    // dari index ke 0 sampai sebelum 5 yaitu 0 sampai 4

    // kalau dari index ke 0 sampai 5
    // pakai =
    let range = 0..=5;

    for i in range {
        println!("{}", arr[i]);
    }

    // OR

    for i in 0..5 {
        println!("{}", arr[i]);
    }

}