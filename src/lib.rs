mod aes;
mod magic;

use rand_mt::Mt64;

// UnityPlayer:$26EA90
fn key_scramble(key: &[u8]) -> Vec<u8> {
    let mut round_keys = vec![0; 11 * 16];
    for round in 0..11 {
        for i in 0..16 {
            for j in 0..16 {
                let idx = (round << 8) + (i * 16) + j;
                round_keys[round * 16 + i] ^=
                    magic::AES_XORPAD_TABLE[1][idx] ^ magic::AES_XORPAD_TABLE[0][idx];
            }
        }
    }

    aes::oqs_mhy128_enc_c(key, &round_keys)
}

// UnityPlayer:$19DA40
fn get_decrypt_vector(key: &[u8], crypt: &[u8]) -> Vec<u8> {
    let crypt: &[u64] = bytemuck::cast_slice(crypt);

    let mut val = 0xFFFFFFFFFFFFFFFF;
    for c in crypt {
        val ^= c;
    }

    let key: &[u64] = bytemuck::cast_slice(key);

    let mut output = vec![0; 4096];
    let mut mt = Mt64::new(key[0] ^ key[1] ^ 0xCEAC3B5A867837AC ^ val);

    mt.fill_bytes(&mut output);

    output
}

pub fn derive(seed: &[u8]) -> Vec<u8> {
    if seed.len() != 2076 {
        panic!("ec2b size must be 2076 (got {})", seed.len());
    }

    let key = seed[8..8 + 16].to_vec();
    let crypt = seed[28..].to_vec();

    let mut key = key_scramble(&key);
    for (i, b) in key.iter_mut().enumerate() {
        *b ^= magic::KEY_XORPAD_TABLE[i];
    }

    get_decrypt_vector(&key, &crypt)
}
