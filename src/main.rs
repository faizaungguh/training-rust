fn main() {
    let  mut pesan_satu: i32 = 1;
    let pesan: &str = "hallo hai";
    println!("pesan ke {}: {}", pesan_satu, pesan);

    pesan_satu = 2; // keyword mut membuat variabel bisa diubah nilainya
    let pesan2: &str = "hallo mut";
    println!("pesan ke {}: {}", pesan_satu, pesan2);
}
