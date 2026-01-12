type Age = u8;
type IdentityNumber = String;

struct Customer {
    id: IdentityNumber,
    name: String,
    age: Age,
}

type Pelanggan = Customer;

fn type_alias() {
    let customer: Pelanggan = Customer {
        id: String::from("001"),
        name: String::from("Wayu"),
        age: 32,
    };

    println!(
        "ID {}, Name: {}, Age: {}",
        customer.id, customer.name, customer.age
    );
}