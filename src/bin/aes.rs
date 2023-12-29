use std::{fs, vec};
use crypto::config;
use crypto::libaes;
use crypto::utils::read_file_to_bytes;


fn main() {
    let data = config::import_config("config.toml");

    let text = read_file_to_bytes(data.config.input_path.as_str());

    let key = read_file_to_bytes(data.config.key_path.as_str());

    let key = key[0..16].try_into().unwrap();

    let block_num = if text.len() % 16 == 0 {
        text.len() / 16
    } else {
        text.len() / 16 + 1
    };

    let mut res = vec![0u8; block_num*16];

    let aes = libaes::AES128::new(key);

    for i in 0..block_num {

        let mut block = [0u8; 16];
        if i == block_num-1 {
            block[0..text.len() - i*16].copy_from_slice(&text[i*16..text.len()]);
        } else {
            block[0..16].copy_from_slice(&text[i*16..(i+1)*16]);
        };
        let block = if data.config.is_encrypt {
            aes.encrypt_block_AES128(&block)
        } else {
            aes.decrypt_block_AES128(&block)
        };

        res[16*i..16*i+16].copy_from_slice(&block);
    }

    fs::write(data.config.output_path, res)
        .expect("Unable to write file");

}
