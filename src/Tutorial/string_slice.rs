fn string_slice() {
    let name: String = String::from("Wah Nov Uto");
    let first_name: &str = &name[0..3]; // dimulai dari char ke 0 sampai ke 2
    let last_name: &str = &name[9..]; // dimulai dari Character ke 8 sampai selesai
}