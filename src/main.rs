fn main() {
    let pesan_satu: i32 = 1;
    let pesan: &str = "hallo hai";
    println!("pesan ke {}: {}", pesan_satu, pesan);

    pesan_satu = 2; // immutable variable akan membatasi perubahan ke variable yang telah berisi nilai sebelumnya, mirip dengan const
    let pesan2: &str = "hallo hai";
    println!("pesan ke {}: {}", pesan_satu, pesan2);
}
