use crate::consts::{E_TABLE, FP_TABLE, IP_TABLE, P_TABLE, PC1, PC2, SBOXES, SHIFTS};

fn bit_shift(src: u64, shift_table: [u8; 64], len: usize) -> u64 {
    let true_len = if len > shift_table.len() { shift_table.len() } else { len };
    let mut res: u64 = 0;
    for i in 0..true_len {
        let bit = (src >> (63-shift_table[i])) & 0x01;
        res |= bit << (63-i);
    }
    res
}


// get the sub key
pub fn get_subkeys(key: u64) -> [u64; 16] {
    let mut keys: [u64; 16] = [0; 16];
    let key = bit_shift(key, PC1, 56);
    let key = key >> 8;

    let mut c = key >> 28;
    let mut d = key & 0x0FFFFFFF;
    for i in 0..16 {
        c = rotate(c, SHIFTS[i]);
        d = rotate(d, SHIFTS[i]);
        keys[i] = bit_shift(((c << 28) | d) << 8, PC2, 48);
    }
    keys
}

///   Performs a left rotate on a 28 bit number
fn rotate(mut val: u64, shift: u8) -> u64 {
    let top_bits = val >> (28 - shift);
    val <<= shift;

    (val | top_bits) & 0x0FFFFFFF
}

fn round(input: u64, key: u64) -> u64 {
    let l = input & (0xFFFF_FFFF << 32);
    let r = input << 32;

    r | ((f(r, key) ^ l) >> 32)
}

fn f(input: u64, key: u64) -> u64 {
    let mut val = bit_shift(input, E_TABLE, 64);
    val ^= key;
    val = s_boxes(val);
    bit_shift(val, P_TABLE, 32)
}

// Applies all eight sboxes to the input
fn s_boxes(input: u64) -> u64 {
    let mut output: u64 = 0;

    for (i, sbox) in SBOXES.iter().enumerate() {
        let val = (input >> (58 - (i * 6))) & 0x3F;
        output |= u64::from(sbox[val as usize]) << (60 - (i * 4));
    }
    output
}


pub fn encrypt(mut data: u64, keys: [u64; 16]) -> u64 {
    data = bit_shift(data, IP_TABLE, 64);

    for key in keys {
        data = round(data, key);
    }
    bit_shift((data << 32) | (data >> 32), FP_TABLE, 64)
}

pub fn decrypt(mut data: u64, keys: [u64; 16]) -> u64 {
    data = bit_shift(data, IP_TABLE, 64);

    for key in keys.iter().rev() {
        data = round(data, *key);
    }
    bit_shift((data << 32) | (data >> 32), FP_TABLE, 64)

}



#[cfg(test)]
mod tests {
    use rand::Rng;
    use super::{encrypt, s_boxes};
    use crate::utils::get_one_bit_num;

    #[test]
    fn test_nolinear() {
        let mut rng = rand::thread_rng();

        for _ in 0..1000 {
            let mut input: [u64; 3] = [0; 3];
            let mut output: [u64; 3] = [0; 3];

            for i in 0..3 {
                input[i] = rng.gen::<u64>();
            }
            for i in 0..3 {
                output[i] = s_boxes(input[i]);
            }

            let k1 = (output[1] as f64 - output[0] as f64) /
                (input[1] as f64 - input[0] as f64);
            let k2 = (output[2] as f64 - output[1] as f64) /
                (input[2] as f64 - input[1] as f64);
            let b1 = output[0] as f64 - k1 * input[0] as f64;
            let b2 = output[1] as f64 - k2 * input[1] as f64;
            // println!("{} {} {} {}", k1, k2, b1, b2);
            assert!((k1 - k2) > 0.0001 || (k2 - k1) > 0.0001 || b1 - b2 > 0.0001 || b1 - b2 < -0.0001);
        }

    }
    #[test]
    fn test_bit_change() {
        let mut rng = rand::thread_rng();

        for _ in 0..1000 {
            let input = rng.gen::<u64>();

            let mut input_change_bit = input;
            input_change_bit ^= 0b1 << (rng.gen::<u8>() % 48 + 16);
            let output = s_boxes(input);
            let output_change_bit = s_boxes(input_change_bit);

            let one_change = output ^ output_change_bit;
            assert!(get_one_bit_num(one_change) > 1);
        }
    }
    #[test]
    fn test_xor_bit_change() {
        let mut rng = rand::thread_rng();

        for _ in 0..1000 {
            let input = rng.gen::<u64>();

            let mut input_change_bit = input;
            for i in 0..8 {
                input_change_bit ^= 0b001100 << (16+(i*6));
            }
            let output = s_boxes(input);
            let output_change_bit = s_boxes(input_change_bit);

            let one_change = output ^ output_change_bit;
            for i in 0..8 {
                let bit_change = get_one_bit_num((one_change >> (60 - (i * 4))) & 0xF);
                assert!(bit_change > 1);
            }
        }
    }
    #[test]
    fn test_gf_bit_change() {
        let mut rng = rand::thread_rng();

        for _ in 0..1000 {
            let input = rng.gen::<u64>();
            let mut input_change_bit = [input; 4];
            let mut output_change_bit = [0; 4];


            for i in 0..8 {
                input_change_bit[0] ^= 0b110100 << (16 + (i * 6));
                input_change_bit[1] ^= 0b111000 << (16 + (i * 6));
                input_change_bit[2] ^= 0b111100 << (16 + (i * 6));
                input_change_bit[3] ^= 0b110000 << (16 + (i * 6));
            }
            let output = s_boxes(input);

            for i in 0..4 {
                output_change_bit[i] = s_boxes(input_change_bit[i]) | output;
            }
            for i in 0..4 {
                for j in 0..8 {
                    let bit_change = get_one_bit_num(
                        (output_change_bit[i] >> (60 - (j * 4))) & 0xF
                    );
                    assert!(bit_change > 0);
                }
            }
        }
    }
    #[test]
    fn test_bit_num_eq() {
        // make one bit fixed
        // and make the other bit random
        // check the zero bit and one bit num is equal
        let get_next_input_num_with_six_bit = |num: u8, fix_bit_idx: u8| -> u8 {
            let bit_mask = 0xffu8.checked_shr((9 - fix_bit_idx) as u32).unwrap_or(0);

            let next_num_value = (((num >> 1) & (0xff << (fix_bit_idx-1))) |
                (num & bit_mask))+1;

            let low =  next_num_value & bit_mask;
            let high =  (next_num_value & (0xff << (fix_bit_idx-1))) << 1;
            high | low | (num & (0b1 << fix_bit_idx-1))
        };
        let get_input_all = |one_input: u8| -> u64 {
            let mut input_all = 0u64;
            for i in 0..8 {
                input_all ^= (one_input as u64) << (16+(i*6));
            }
            input_all
        };
        for i in 0..6 {
            let mut one_bit_num_with_zero = 0i32;
            let mut one_bit_num_with_one = 0i32;
            let mut input_one = (0b1 << i) as u8;
            let mut input_zero = 0u8;
            for j in 0..32 {
                let input_one_all = get_input_all(input_one)  ;
                let input_zero_all = get_input_all(input_zero);
                let output_one = s_boxes(input_one_all);
                let output_zero = s_boxes(input_zero_all);
                one_bit_num_with_one += get_one_bit_num(output_one) as i32;
                one_bit_num_with_zero += get_one_bit_num(output_zero) as i32;
                if j != 31 {
                    input_one = get_next_input_num_with_six_bit(input_one, i + 1);
                    input_zero = get_next_input_num_with_six_bit(input_zero, i + 1);
                }
            }
            assert!((one_bit_num_with_one - (4*8*32-one_bit_num_with_one)).abs() < 50);
            assert!((one_bit_num_with_zero  - (4*8*32-one_bit_num_with_zero)).abs() < 50);

        }
    }
    #[test]
    fn test_gen_keys() {
        let key = 0b00110001_00110010_00110011_00110100_00110101_00110110_00110111_00111000;
        let keys = super::get_subkeys(key);
        let data = 0b10001011_10110100_01111010_00001100_11110000_10101001_01100010_01101101;
        let src_data = super::decrypt(data, keys);

        assert_eq!(src_data,
                   0b00110000_00110001_00110010_00110011_00110100_00110101_00110110_00110111);
    }
}
