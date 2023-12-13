use std::fs::File;
use std::io::Read;


pub fn read_file_to_bytes(path: &str) -> Vec<u8> {
    let mut f = File::open(path).expect("文件打开失败");
    let metadata = std::fs::metadata(path).expect("无法读取元数据");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("文件读取失败");

    buffer
}
