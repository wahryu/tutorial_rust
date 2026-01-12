fn main() {
    println!("Hello, world!");
    let num: i32 = 10;
    println!("{}", num);
}

mod tutorial::module::first::sayhello;

use first::sayhello;
use second::sayhello as sayhello2;

#[test]
fn test_modd() {
    sayhello(); // first module
    sayhello2(); // second module
}