use crypto::banner;
use crypto::shift::shift_char;

fn main() {
    let  (encrypt, input, key) = banner::banner();

    match encrypt {
        true => {
            println!("Encrypted: {}", vigenere_encrypt(input.trim_end(), key.trim_end()));
        },
        false => {
            println!("Decrypted: {}", vigenere_decrypt(input.trim_end(), key.trim_end()));
        }
    }

}

fn vigenere_encrypt(text: &str, key: &str) -> String {
    transform(text, key, true)
}

fn vigenere_decrypt(text: &str, key: &str) -> String {
    transform(text, key,false)
}

fn transform(text: &str, key: &str, encrypt: bool) -> String {
    let key_len = key.len();
    let mut result = String::new();

    let mut alpha_count = 0;
    for c in text.chars() {
        // if !c.is_ascii_alphabetic() {
        //     result.push(c);
        //     continue;
        // }
        let key_char = key.chars().nth(alpha_count % key_len).unwrap();
        alpha_count += 1;
        let key_shift = key_char as u8 - 'a' as u8;

        let new_char = if encrypt {
            shift_char(c, key_shift)
        } else {
            shift_char(c, 26 - key_shift)
        };

        result.push(new_char);
    }

    result

}