/*
Nama modul huruf kecil semua : snake_case
kalau ingin di akses dari luar gunakan "pub" di depan

Kalau ingin membuat modul di file terpisah, nama file = nama modul
jadi tidak perlu menggunakan "mod".
kecuali ingin membuat sub modul
*/

pub mod model {
    struct User {
        first: String,
        last: String,
        age: u8,
    }

    impl User {
        pub fn hello(&self, name: &str) {
            println!("{} {}", name, self.first);
        }
    }
}

/* 
untuk mengakses module
nama_module::function
*/

fn test_module() {
    let user: model::User = model::User{
        first: String::from("Wah"),
        last: String::from("Uto"),
        age: 30
    };

    user.hello("Budi");
}

//=================================================

mod first {
    pub fn shello() {
        println!("Hello first");
    }
}

mod second {
    pub fn shello() {
        println!("Hello second");
    }
}

use first::shello;
use second::shello as shello2;

fn test_mod() {
    shello();
    sshello2();
}