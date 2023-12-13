use crypto::banner;
use crypto::shift::shift_char;
fn main() {

    let (encrypt, input, key) = banner::banner();

    let key: u8 = key
        .trim()
        .parse()
        .expect("Invalid shift key");

    match encrypt {
        true => {
            println!("Encrypted: {}", caesar_encrypt(&input, key));
        },
        false => {
            println!("Decrypted: {}", caesar_decrypt(&input, key));
        }
    }

}

fn caesar_encrypt(text: &str, key: u8) -> String {
    text.chars()
        .map(|c| shift_char(c, key))
        .collect()
}

fn caesar_decrypt(text: &str, key: u8) -> String {
    text.chars()
        .map(|c| shift_char(c, 26 - key))
        .collect()
}

