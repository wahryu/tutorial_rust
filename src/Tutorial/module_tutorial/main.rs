mod tutorial;

use tutorial::module_tutorial::first::sayhello;
use tutorial::module_tutorial::second::sayhello as sayhello2;

#[test]
fn test_modd() {
    sayhello(); // first module
    sayhello2(); // second module
}