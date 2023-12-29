use std::fs;
use crypto::{config, libsm4};
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

        let sm4 = libsm4::SM4::new(key);

        for i in 0..block_num {

            let mut block = [0u8; 16];
            if i == block_num-1 {
                block[0..text.len() - i*16].copy_from_slice(&text[i*16..text.len()]);
            } else {
                block[0..16].copy_from_slice(&text[i*16..(i+1)*16]);
            };
            let block = if data.config.is_encrypt {
                sm4.encrypt(&block)
            } else {
                sm4.decrypt(&block)
            };

            res[16*i..16*i+16].copy_from_slice(&block);
        }

        fs::write(data.config.output_path, res)
            .expect("Unable to write file");

}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_sm4() {
        let input : [u8; 16] = [0x01, 0x23, 0x45, 0x67,
            0x89, 0xab, 0xcd, 0xef,
            0xfe, 0xdc, 0xba, 0x98,
            0x76, 0x54, 0x32, 0x10];
        let key : [u8; 16] = [0x01, 0x23, 0x45, 0x67,
            0x89, 0xab, 0xcd, 0xef,
            0xfe, 0xdc, 0xba, 0x98,
            0x76, 0x54, 0x32, 0x10];
        let sm4 = libsm4::SM4::new(&key);
        let output = sm4.encrypt(&input);
        println!("{:?}", output);
    }

}