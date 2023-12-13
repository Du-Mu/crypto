use crypto::banner;

fn main() {
    let  (encrypt, input, key) = banner::banner();

    let key: usize = key
        .trim()
        .parse()
        .expect("Invalid shift key");

    match encrypt {
        true => {
            println!("Encrypted: {}", fence_encrypt(input.trim_end(), key));
        },
        false => {
            println!("Decrypted: {}", fence_decrypt(input.trim_end(), key));
        }
    }

}

fn fence_encrypt(plaintext: &str, rail_count: usize) -> String {
    let row = plaintext.len().wrapping_div(rail_count);
    let size = row*rail_count;
    let mut result_vec = vec!['\x7f'; size];
    let mut result = String::new();

    let mut index = 0;

    for i in 0..row {
        for j in 0..rail_count {
            if index < plaintext.len() {
                result_vec[j*row + i] = plaintext.chars().nth(index).unwrap();
                index += 1;
            }
        }
    }
    for i in 0..row*rail_count {
        if result_vec[i] != '\x7f' {
            result.push(result_vec[i])
        }
    }
    result
}

fn fence_decrypt(ciphertext: &str, rail_count: usize) -> String {
    let row = ciphertext.len().wrapping_div(rail_count);
    let size = row*rail_count;
    let d_size = size - ciphertext.len();
    let mut result_vec = vec!['\x7f'; size];
    let mut result = String::new();

    let mut index = 0;

    for i in 0..row {
        let mut cipher_idx = 0;
        for j in 0..rail_count {
            result_vec[index] = ciphertext.chars().nth(cipher_idx+i).unwrap();
            if j < rail_count- d_size && index < ciphertext.len() {
                cipher_idx += row;
            } else {
                cipher_idx += row - 1;
            };
            index += 1;
        }
    }
    for i in 0..row*rail_count {
        if result_vec[i] != '\x7f' {
            result.push(result_vec[i])
        }
    }
    result
}
