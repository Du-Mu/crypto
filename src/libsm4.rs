use crate::consts::{SM4_SBOX, SM4_FK, SM4_CK};

macro_rules! GETU32 {
    ($ptr: expr) => {
        {
            (u32::from(($ptr)[0]) << 24) |
            (u32::from(($ptr)[1]) << 16) |
            (u32::from(($ptr)[2]) << 8) |
            u32::from(($ptr)[3])
        }
    };
}

macro_rules! PUTU32 {
    ($ptr: expr, $value: expr) => {
        {
            ($ptr)[0] = (($value) >> 24) as u8;
            ($ptr)[1] = (($value) >> 16) as u8;
            ($ptr)[2] = (($value) >> 8) as u8;
            ($ptr)[3] = ($value) as u8;
        }
    };
}

macro_rules! ROL32 {
    ($X: expr, $n: expr) => {
        ((($X)<<$n) | (($X)>>(32-$n)))
    };
}

macro_rules! L32 {
    ($X: expr) => {
        (($X) ^ ROL32!(($X), 2) ^ ROL32!(($X), 10) ^ ROL32!(($X), 18) ^ ROL32!(($X), 24))
    };
}

macro_rules! L32_2 {
    ($X: expr) => {
        ( ($X) ^
        ROL32!(($X), 13) ^
        ROL32!(($X), 23) )
    };
}

macro_rules! S32 {
    ($A: expr) => {
        {
            (SM4_SBOX[(($A) >> 24) as usize] as u32) << 24 |
            (SM4_SBOX[(((($A) >> 16) & 0xff) as usize)] as u32)  << 16 |
            (SM4_SBOX[(((($A) >> 8) & 0xff) as usize)]  as u32)  << 8 |
            (SM4_SBOX[(($A) & 0xff) as usize] as u32)
        }
    };
}

pub struct SM4 {
    encrypt_key: [u32; 32]
}

impl SM4 {
    pub fn new(keys: &[u8; 16]) -> SM4 {
        return SM4 {
            encrypt_key: sm4_gen_encrypt_keys(keys),
        }
    }
    pub fn encrypt(&self, input: &[u8; 16]) -> [u8; 16] {
        let mut X: [u32; 5] = [0; 5];

        X[0] = GETU32!(input);
        X[1] = GETU32!(&input[4..8]);
        X[2] = GETU32!(&input[8..12]);
        X[3] = GETU32!(&input[12..16]);

        for i in 0..32 {
            X[4] = X[1] ^ X[2] ^ X[3] ^ self.encrypt_key[i];
            X[4] = S32!(X[4]);
            X[4] = X[0] ^ L32!(X[4]);

            X[0] = X[1];
            X[1] = X[2];
            X[2] = X[3];
            X[3] = X[4];
        }
        let mut res = [0u8; 16];
        PUTU32!(&mut res, X[3]);
        PUTU32!(&mut res[4..8], X[2]);
        PUTU32!(&mut res[8..12], X[1]);
        PUTU32!(&mut res[12..16], X[0]);

        res
    }

    pub fn decrypt(&self, input: &[u8; 16]) -> [u8; 16] {
        let mut X: [u32; 5] = [0; 5];

        X[0] = GETU32!(input)          ^ SM4_FK[0];
        X[1] = GETU32!(&input[4..8])   ^ SM4_FK[1];
        X[2] = GETU32!(&input[8..12])  ^ SM4_FK[2];
        X[3] = GETU32!(&input[12..16]) ^ SM4_FK[3];

        for (i, k) in self.encrypt_key.iter().rev().enumerate() {
            X[4] = X[1] ^ X[2] ^ X[3] ^ k;
            X[4] = S32!(X[4]);
            X[4] = X[0] ^ L32!(X[4]);

            X[0] = X[1];
            X[1] = X[2];
            X[2] = X[3];
            X[3] = X[4];
        }

        let mut res = [0u8; 16];
        PUTU32!(&mut res, X[3]);
        PUTU32!(&mut res[4..8], X[2]);
        PUTU32!(&mut res[8..12], X[1]);
        PUTU32!(&mut res[12..16], X[0]);

        res

    }

}

fn sm4_gen_encrypt_keys(uk: &[u8; 16]) -> [u32; 32] {
    let mut X: [u32; 5] = [0; 5];
    let mut keys: [u32; 32] = [0; 32];
    X[0] = GETU32!(uk)          ^ SM4_FK[0];
    X[1] = GETU32!(&uk[4..8])   ^ SM4_FK[1];
    X[2] = GETU32!(&uk[8..12])  ^ SM4_FK[2];
    X[3] = GETU32!(&uk[12..16]) ^ SM4_FK[3];

    for i in 0..32 {
        X[4] = X[1] ^ X[2] ^ X[3] ^ SM4_CK[i];
        X[4] = S32!(X[4]) ;
        X[4] = X[0] ^ L32_2!(X[4]);

        keys[i] = X[4];

        X[0] = X[1];
        X[1] = X[2];
        X[2] = X[3];
        X[3] = X[4];
    }
    return keys;
}