use crypto_prices::get_hash;

fn main() {
    let lala = get_hash(String::from("hola"));
    println!("{lala}");
}
