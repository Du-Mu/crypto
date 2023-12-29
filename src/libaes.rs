use std::convert::AsMut;
use crate::consts::{AES_SBOX, EXP_TABLE, INVERSE_AES_SBOX, LOG_TABLE,  RC};
use crate::utils::{addition_gf, multiplication_gf};


macro_rules! round {
    (
        $state: ident,
        $aes128: ident,
        $i: ident
    ) => {
        sub_bytes(&mut $state);
        shift_rows(&mut $state);
        mix_columns(&mut $state);
        add_round_key(&mut $state, &clone_into_array(& $aes128.expanded_key[$i*4..($i+1)*4]));
    };
}


pub struct AES128 {
    expanded_key: [[u8;4];44],
}


impl AES128 {
    pub fn new(key: &[u8; 16]) -> AES128 {
        return AES128 {
            expanded_key: key_schedule_AES128(key),
        }
    }
    pub fn decrypt_block_AES128(&self, bytes: &[u8;16]) -> [u8;16] {
        let mut result = [0u8;16];

        let mut state = [[0u8;4];4];
        for i in 0..16 {
            state[i%4][i/4] = bytes[i];
        }

        add_round_key(&mut state, &clone_into_array(&self.expanded_key[40..44]));
        inv_shift_rows(&mut state);
        inv_sub_bytes(&mut state);

        for i in (1..10).rev() {
            add_round_key(&mut state, &clone_into_array(&self.expanded_key[i*4..(i+1)*4]));
            inv_mix_columns(&mut state);
            inv_shift_rows(&mut state);
            inv_sub_bytes(&mut state);
        }

        add_round_key(&mut state, &clone_into_array(&self.expanded_key[0..4]));

        for i in 0..4 {
            for j in 0..4 {
                result[4*j+i] = state[i][j]
            }
        }

        return result;
    }
    pub fn encrypt_block_AES128(&self, bytes: &[u8;16]) -> [u8;16] {
        let mut result = [0u8;16];

        let mut state = [[0u8;4];4];
        for i in 0..16 {
            state[i%4][i/4] = bytes[i];
        }

        add_round_key(&mut state, &clone_into_array(&self.expanded_key[0..4]));

        for i in 1..10 {
            round!(state, self, i);
        }

        sub_bytes(&mut state);
        shift_rows(&mut state);
        add_round_key(&mut state, &clone_into_array(&self.expanded_key[40..44]));

        for i in 0..4 {
            for j in 0..4 {
                result[4*j+i] = state[i][j]
            }
        }

        return result;
    }

}

fn clone_into_array<A, T>(slice: &[T]) -> A
    where
        A: Default + AsMut<[T]>,
        T: Clone,
{
    let mut a = A::default();
    <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
    a
}



fn key_schedule_AES128(key_bytes: &[u8;16]) -> [[u8;4];44] {
    let mut original_key = [[0u8;4];4];
    let mut expanded_key = [[0u8;4];44];
    let N = 4;

    for i in 0..16 {
        original_key[i/4][i%4] = key_bytes[i];
    }

    for i in 0..44 { // 11 rounds, i in 0..4*rounds-1

        if i < N {
            expanded_key[i] = original_key[i];
        } else if  i >= N && i % N == 0 {

            let mut rcon = [0u8;4];
            rcon[0] = RC[i/N];
            expanded_key[i] = addition_gf(&addition_gf(&expanded_key[i-N], &sub_word(&rot_word(&expanded_key[i-1]))), &rcon);

        } else {
            expanded_key[i] = addition_gf(&expanded_key[i-N], &expanded_key[i-1]);
        }

    }

    return expanded_key;
}

fn substitute(byte: u8, encryption: bool) -> u8 {
    let upper_nibble : usize;
    let lower_nibble : usize;
    upper_nibble = ((byte>>4) & 0xF).into();
    lower_nibble = (byte & 0xF).into();
    if encryption == true {
        return AES_SBOX[upper_nibble][lower_nibble];
    } else {
        return INVERSE_AES_SBOX[upper_nibble][lower_nibble];
    }
}

fn rot_word(word: &[u8; 4]) -> [u8;4] {
    let mut result = [0u8;4];

    for i in 0..4 {
        result[i] = word[(i+1)%4];
    }

    return result;
}

fn sub_word(word: &[u8; 4]) -> [u8;4] {
    let mut result = [0u8;4];

    for i in 0..4 {
        result[i] = substitute(word[i], true);
    }

    return result;
}



fn add_round_key(state:&mut [[u8;4];4], key: &[[u8;4];4]) {
    for i in 0..4 {
        for j in 0..4 {
            state[i][j] = state[i][j] ^ key[j][i];
        }
    }
}

fn sub_bytes(state:&mut [[u8;4];4]) {
    for i in 0..4 {
        for j in 0..4 {
            state[i][j] = substitute(state[i][j], true);
        }
    }
}

fn inv_sub_bytes(state:&mut [[u8;4];4]) {
    for i in 0..4 {
        for j in 0..4 {
            state[i][j] = substitute(state[i][j], false);
        }
    }
}

fn shift_rows(state:&mut [[u8;4];4]) {
    for i in 1..4 {
        let mut tmp = vec![0u8;i];
        for j in 0..i {
            tmp[j] = state[i][j];
        }
        for j in 0..4-i {
            state[i][j] = state[i][j+i];
        }
        for j in 0..i {
            state[i][3-j] = tmp[i-j-1];
        }
    }
}

fn inv_shift_rows(state:&mut [[u8;4];4]) {
    for i in (1..4).rev() {
        let mut tmp = vec![0u8;i];
        for j in 0..i {
            tmp[j] = state[4-i][j];
        }
        for j in 0..4-i {
            state[4-i][j] = state[4-i][j+i];
        }
        for j in 0..i {
            state[4-i][3-j] = tmp[i-j-1];
        }
    }
}

fn mix_columns(state: &mut [[u8;4];4]) {
    for i in 0..4 {

        let mut temp = [0u8;4];
        for j in 0..4 {
            temp[j] = state[j][i];
        }

        state[0][i] = multiplication_gf(temp[0], 2) ^ multiplication_gf(temp[3], 1) ^ multiplication_gf(temp[2], 1) ^ multiplication_gf(temp[1], 3);
        state[1][i] = multiplication_gf(temp[1], 2) ^ multiplication_gf(temp[0], 1) ^ multiplication_gf(temp[3], 1) ^ multiplication_gf(temp[2], 3);
        state[2][i] = multiplication_gf(temp[2], 2) ^ multiplication_gf(temp[1], 1) ^ multiplication_gf(temp[0], 1) ^ multiplication_gf(temp[3], 3);
        state[3][i] = multiplication_gf(temp[3], 2) ^ multiplication_gf(temp[2], 1) ^ multiplication_gf(temp[1], 1) ^ multiplication_gf(temp[0], 3);

    }
}

fn inv_mix_columns(state: &mut [[u8;4];4]) {
    for i in 0..4 {

        let mut temp = [0u8;4];
        for j in 0..4 {
            temp[j] = state[j][i];
        }

        state[0][i] = multiplication_gf(temp[0], 14) ^ multiplication_gf(temp[3], 9) ^ multiplication_gf(temp[2], 13) ^ multiplication_gf(temp[1], 11);
        state[1][i] = multiplication_gf(temp[1], 14) ^ multiplication_gf(temp[0], 9) ^ multiplication_gf(temp[3], 13) ^ multiplication_gf(temp[2], 11);
        state[2][i] = multiplication_gf(temp[2], 14) ^ multiplication_gf(temp[1], 9) ^ multiplication_gf(temp[0], 13) ^ multiplication_gf(temp[3], 11);
        state[3][i] = multiplication_gf(temp[3], 14) ^ multiplication_gf(temp[2], 9) ^ multiplication_gf(temp[1], 13) ^ multiplication_gf(temp[0], 11);
    }
}



#[cfg(test)]
mod test {
    use std::iter::once_with;
    use std::ptr::null_mut;
    use rand::Rng;
    use crate::libaes::{AES128,mix_columns, substitute};

    fn get_one_byte_num(num: u8) -> u8 {
        let mut res = 0;
        for j in 0..8 {
            if (num >> j) & 0x01 == 1 {
                res += 1;
            }
        }
        res
    }

    #[test]
    fn test_output() {
        let mut rng = rand::thread_rng();
        let mut change_num = 0u64;

        for _ in 0..1000 {
            let key = rng.gen::<[u8; 16]>();
            let aes = super::AES128::new(&key);

            let mut input = rng.gen::<[u8; 16]>();
            let output =
                aes.encrypt_block_AES128(&input);

            let rand_change_byte = rng.gen::<u8>() % (16 * 8);

            input[rand_change_byte as usize / 8] ^= 0b1 << (rand_change_byte % 8);

            let output_change =
                aes.encrypt_block_AES128(&input);

            for j in 0..16 {
                let change = output[j] ^ output_change[j];
                change_num += get_one_byte_num(change) as u64;
            }
        }
        println!("output_change:{}", change_num as f64 /1000.0);
    }
    #[test]
    fn test_sbox_output() {
        let mut rng = rand::thread_rng();
        let mut change_num = 0u64;

        for _ in 0..1000 {
            let mut input = rng.gen::<u8>();
            let output =
                substitute(input.clone(), true);

            let rand_change_byte = rng.gen::<u8>() %  8;

            input ^= 0b1 << rand_change_byte;

            let output_change =
                substitute(input.clone(), true);

            let change = output ^ output_change;
            change_num += get_one_byte_num(change) as u64;

        }
        println!("S_box_change:{}", change_num as f64 /1000.0);
    }
    #[test]
    fn test_mix_column() {
        let mut rng = rand::thread_rng();
        let mut change_num = 0u64;

        for _ in 0..1000 {
            let mut input = rng.gen::<[[u8; 4]; 4]>();
            let mut input_clone = input.clone();

            let rand_change_byte = rng.gen::<u8>() %  (32*4);

            input_clone[rand_change_byte as usize / 32][rand_change_byte as usize % 32 / 8]
                ^= 0b1 << (rand_change_byte as usize % 8);

            mix_columns(&mut input);
            mix_columns(&mut input_clone);

            for i in 0..4 {
                for j in 0..4 {
                    let change = input[i][j] ^ input_clone[i][j];
                    change_num += get_one_byte_num(change) as u64;
                }
            }
        }
        println!("mix_column_change:{}", change_num as f64 /1000.0);

    }

    #[test]
    fn test_sbox_transformer() {
        let mut rng = rand::thread_rng();
        let mut trans_num = 0u64;
        for _ in 0..1000 {
            let mut tmp_trans = 0u64;
            let mut input = rng.gen::<u8>();
            let mut output =
                substitute(input.clone(), true);
                tmp_trans += 1;
            while output != input {
                output = substitute(output.clone(), true);
                tmp_trans += 1;
            }
            trans_num += tmp_trans;
        }
        println!("trans_num:{}", trans_num as f64 /1000.0);
    }

    #[test]
    fn differential_distribution_table() {
        let mut ddt: [[u16; 0x100]; 0x100] = [[0; 0x100]; 0x100];

        for i in 0..0x100 {
            for j in 0..0x100 {
                let output1 = substitute(i as u8, true);
                let output2 = substitute(j as u8, true);
                let dd_in = i ^ j;
                let dd_out = output1 ^ output2;
                ddt[dd_out as usize][dd_in as usize] += 1;
            }
        }
        // println!("ddt: {:?}", ddt);
    }
    fn poppar(x: u8) -> u8 {
        let mut res = 0;
        for i in 0..8 {
            res ^= (x >> i) & 0x01;
        }
        res
    }

    fn get_linear(a: u8, b: u8) -> i32 {
        let mut count = 0;
        for i in 0..0x100u16 {
            if poppar( a & i as u8)  == poppar(substitute(i as u8, true) & b) {
                count += 1;
            }
        }
        count -= 128;
        count

    }
    #[test]
    fn lineaer_table() {
        let mut lt: [[i32; 0x100]; 0x100] = [[0; 0x100]; 0x100];

        for i in 0..0x100 {
            for j in 0..0x100 {
                lt[i][j] = get_linear(i as u8, j as u8);
            }
        }
        println!("lt: {:?}", lt);
    }
}

