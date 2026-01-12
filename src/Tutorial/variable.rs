/*
Immutable Variabel (variabel yang tidak bisa dirubah)
STACK
*/
fn immutable_variable() {
    // INTEGER
    let num: i32 = -10; // i8 ~ i128

    // REAL NUMBER
    let real_num: u32 = 10; // u8 ~ u128

    // FLOAT
    let decimal: f64 = 20.5; // f8 ~ f128

    // STRING
    let text: &str = "STRing";

    // CHAR
    // Only One Character
    let char: char = '1';

    // TUPLE
    // Multiple Type Data OK
    // Access = variable.INDEX
    let tup: (i32, u8, f64, &str) = (-10, 10, 10,5, "text");

    // ARRAY
    // Only One Type Data
    // Access = variable[INDEX]
    let array: [i32; 5] = [1, 2, 3, 4, 5]; // Type Data INTEGER, Length = 5

    // BOOLEAN
    let bool_true: bool = true;
    let bool_false: bool = false;

    // CONSTANTA
    const pi: f64 = 3.14;
}

/*
Mutable Variabel (variabel yang tidak bisa dirubah)
STACK

walaupun digunakan di variabel lain, masih bisa di gunakan
*/
fn mutable_variable_stack() {
    // INTEGER
    let mut num: i32 = -10; // i8 ~ i128

    // REAL NUMBER
    let mut real_num: u32 = 10; // u8 ~ u128

    // FLOAT
    let mut decimal: f64 = 20.5; // f8 ~ f128

    // STRING
    let mut text: &str = "STRing";

    // CHAR
    // Only One Character
    let mut char: char = '1';

    // TUPLE
    // Multiple Type Data OK
    // Access = variable.INDEX
    let mut tup: (i32, u8, f64, &str) = (-10, 10, 10,5, "text");

    // ARRAY
    // Only One Type Data
    // Access = variable[INDEX]
    let mut array: [i32; 5] = [1, 2, 3, 4, 5]; // Type Data INTEGER, Length = 5

    // BOOLEAN
    let mut bool_true: bool = true;
}

/*
Mutable Variabel (variabel yang tidak bisa dirubah)
HEAP
variable ini tidak bisa di masukkan di variabel lain
karena value nya akan berpindah ke variable yang baru
jadi variabel lama sudah tidak bisa digunakan lagi

kalau masih ingin menggunakan variabel pertama, maka gunakan .clone()
*/
fn mutable_variable_heap() {
    // STRING
    let text: String = String::from("Mutable");

    let tex2: String = text;

    let tex3: String = tex2.clone();

    let ref_text: &String = tex2; // ini adalah reference. jadi value di tex2 bisa digunakan juga

    println!("{}", text); // Error disini karena sudah berpindah ke tex2
    println!("{}", tex2);
}