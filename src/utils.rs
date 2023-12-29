use std::fs::File;
use std::io::Read;
use crate::consts::{EXP_TABLE, LOG_TABLE};


pub fn read_file_to_bytes(path: &str) -> Vec<u8> {
    let mut f = File::open(path).expect("文件打开失败");
    let metadata = std::fs::metadata(path).expect("无法读取元数据");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("文件读取失败");

    buffer
}

pub fn get_one_bit_num(num: u64) -> u8 {
    let mut res = 0;
    for i in 0..64 {
        if (num >> i) & 0x01 == 1 {
            res += 1;
        }
    }
    res
}

pub fn multiplication_gf(a: u8, b: u8) -> u8 {
    let (sum, _) = LOG_TABLE[a as usize].overflowing_add( LOG_TABLE[b as usize]);

    return EXP_TABLE[(sum % 0xff) as usize]
}
pub fn addition_gf(equ1: &[u8; 4], equ2: &[u8; 4]) -> [u8;4] {
    let mut result = [0u8;4];

    for i in 0..4 {
        result[i] = equ1[i] ^ equ2[i];
    }
    return result;
}