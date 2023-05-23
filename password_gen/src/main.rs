use rand::{Rng};
use std::env;



fn main() {
    let args: Vec<String> = env::args().collect();
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    let number: usize = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input");
            return;
        }
    };
    println!("number: {}", number);
    let mut rng = rand::thread_rng();
    let password: String = (0..number)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("{}", password);
}
