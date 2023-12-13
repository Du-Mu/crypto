use std::fs;
use criterion::{criterion_group, criterion_main, Criterion};
use crypto::config;
use crypto::libdes::{decrypt, encrypt, get_subkeys};
use crypto::utils::read_file_to_bytes;

extern crate crypto;

fn bench_encrypt() {
    let data = config::import_config("config.toml");

    let text = read_file_to_bytes(data.config.input_path.as_str());

    let key = read_file_to_bytes(data.config.key_path.as_str());
    let key = u64::from_be_bytes(key[0..8].try_into().unwrap());
    let block_num = if text.len() % 8 == 0 {
        text.len() / 8
    } else {
        text.len() / 8 + 1
    };

    let mut res = vec![0u8; block_num*8];

    let keys = get_subkeys(key);

    for i in 0..block_num {
        let block = if i == block_num-1 {
            let mut block = [0u8; 8];
            block[0..text.len() - i*8].copy_from_slice(&text[i*8..text.len()]);
            u64::from_be_bytes(block)
        } else {
            u64::from_be_bytes(text[i*8..(i+1)*8].try_into().unwrap())
        };
        let block = if data.config.is_encrypt {
            encrypt(block, keys)
        } else {
            decrypt(block, keys)
        };
        res[8*i..8*i+8].copy_from_slice(&block.to_be_bytes());
    }

    fs::write(data.config.output_path, res)
        .expect("Unable to write file");

}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("des: ", |b| b.iter(|| bench_encrypt()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);