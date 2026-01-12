/*
Struct mirip dict atau dataclass di Python
menggunaman CamelCase
*/

struct Person {
    first_name : String,
    middle_name : String,
    last_name : String,
    age : i32,
}

fn print_person(person: &Person) {
    println!("{}", person.first_name);
    println!("{}", person.midle_name);
    println!("{}", person.lase_name);
    println!("{}", person.age);
}

fn struct_person() {
    let person: Person = Person {
        first_name: String::from("Wah"),
        midle_name: String::from("Wah"),
        last_name: String::from("Wah"),
        age: 32,
    };

    print_person(&person);
}