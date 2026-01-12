struct Person {
    first_name : String,
    middle_name : String,
    last_name : String,
    age : i32,
}

impl Person {
    fn hello(&self, name: &str) {
        println!("Hello {}, saya {}", name, self.first_name);
    }
}

fn struct_person() {
    let person: Person = Person {
        first_name: String::from("Wah"),
        midle_name: String::from("Wah"),
        last_name: String::from("Wah"),
        age: 32,
    };

    person.hello("Budi");
}