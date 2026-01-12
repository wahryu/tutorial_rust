fn match_value() {
    let name: String = String::from("Budi");

    match name {
        "Budi" => {
            println!("Budi");
        }
        "Eko" => {
            println!("Eko");
        }
        //butuh variable
        other => {
            println1("{}", other);
        }
    }
}

fn match_range() {
    let nilai: i32 = 50;

    match nilai {
        75..=100 => {
            println!("A");
        }
        60..=74 => {
            println!("B");
        }
        30..=59 => {
            println!("C");
        }
        // tidak butuh variable
        _ => {
            println!("D");
        }
    }
}

fn match_ex() {
    let val: i32 = 2;

    let result: &str = match value {
        0 => "nol",
        1 => "satu",
        2 => "dua",
        _ => "invalid"
    };

    println!("{}", result);
}