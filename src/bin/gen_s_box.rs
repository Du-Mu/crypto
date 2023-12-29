// 题目2：S盒的设计：产生新的S盒使其达到题1中的性质最优；
// 例：AES的S盒是计算输入X的逆，然后做仿射变换得出输出Y=AX-1+B= AX254+B。
// 尝试Y=AXC+B的形式,
// (1)C要求汉明重量为7(例如AES中254=11111110),
// (2)新盒可以改变仿射变换使用的（满秩）矩阵A或向量B
// 给出结果,并计算其差分分布表和非线性度.
//
// C: 1111101
// Affine Matrix:
// 1 1 0 0 0 0 0 1 [b0      [0
// 1 1 1 0 0 0 0 0  b1       0
// 0 1 1 1 0 0 0 0  b2       1
// 0 0 1 1 1 0 0 0  b3 xor   0
// 0 0 0 1 1 1 0 0  b4       1
// 0 0 0 0 1 1 1 0  b5       0
// 0 0 0 0 0 1 1 1  b6       1
// 1 0 0 0 0 0 1 1  b7]      0]


use crypto::utils::{addition_gf, multiplication_gf};

const MATRIX: [[u8; 8]; 8] = [
[1, 1, 0, 0, 0, 0, 0, 1],
[1, 1, 1, 0, 0, 0, 0, 0],
[0, 1, 1, 1, 0, 0, 0, 0],
[0, 0, 1, 1, 1, 0, 0, 0],
[0, 0, 0, 1, 1, 1, 0, 0],
[0, 0, 0, 0, 1, 1, 1, 0],
[0, 0, 0, 0, 0, 1, 1, 1],
[1, 0, 0, 0, 0, 0, 1, 1],
];

fn main() {
    let mut s_box: [u8; 256] = [0; 256];

    for i in 0..256 {
        let mut res = 0;
        for _ in 0..0b11111101  {
            res = res ^ i as u8 ;
        }
        s_box[i] = affineTransform(res);

    }
    println!("{:?}", s_box);
}
fn affineTransform(input: u8) -> u8 {
    let mut res = 0;
    finite_field_matrix_multiply(&input);
    res ^= 0b00101010;
    res

}

fn finite_field_matrix_multiply(vector: &u8) -> u8 {
    let mut result = 0u8;
    let mut result_vector = [0; 8];
    let mut vector_2 = [0; 8];
    for i in 0..8 {
        vector_2[i] = vector>>i & 0b1;
    }

    for i in 0..8 {
        for j in 0..8 {
            result_vector[i] ^= multiplication_gf(MATRIX[i][j], vector_2[j]);
        }
    }

    for i in 0..8 {
        result ^= (result_vector[i]&0b1) <<i;
    }
    result

}
