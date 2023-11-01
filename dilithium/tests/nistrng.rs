use aes::{
    cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit, Unsigned},
    Aes256Enc,
};
use signature::rand_core::{CryptoRng, RngCore};

type SeedLen = generic_array::typenum::U48;
type KeyLen = generic_array::typenum::U32;
type BlockLen = generic_array::typenum::U16;

#[derive(Clone, Debug)]
pub struct RandomBytes {
    pub(crate) key: GenericArray<u8, KeyLen>,
    pub(crate) v: GenericArray<u8, BlockLen>,
    pub(crate) reseed_counter: u64,
}

impl RandomBytes {
    pub fn init(entropy_input: &[u8; SeedLen::USIZE]) -> Self {
        let mut key = Default::default();
        let mut v = Default::default();
        aes256_ctr_drbg_update(Some(entropy_input.into()), &mut key, &mut v);
        RandomBytes {
            key,
            v,
            reseed_counter: 1,
        }
    }
}

impl RngCore for RandomBytes {
    fn next_u32(&mut self) -> u32 {
        panic!("not supported")
    }

    fn next_u64(&mut self) -> u64 {
        panic!("not supported")
    }

    fn fill_bytes(&mut self, bytes: &mut [u8]) {
        let mut i = 0;
        let fill_len = bytes.len();
        while i < fill_len {
            //increment V
            for j in (0..16).rev() {
                if self.v[j] == 0xff {
                    self.v[j] = 0x00;
                } else {
                    self.v[j] += 1;
                    break;
                }
            }
            let cipher = Aes256Enc::new(&self.key.clone().into());
            let mut block: GenericArray<u8, BlockLen> = Default::default();
            block.copy_from_slice(&self.v);
            cipher.encrypt_block(&mut block);
            let is_last_block = i + 16 > fill_len;
            if is_last_block {
                bytes[i..].copy_from_slice(&block[0..fill_len - i]);
                i = fill_len;
            } else {
                bytes[i..i + 16].copy_from_slice(&block);
                i += 16;
            }
        }
        aes256_ctr_drbg_update(None, &mut self.key, &mut self.v);
        self.reseed_counter += 1;
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), signature::rand_core::Error> {
        Ok(self.fill_bytes(dest))
    }
}

impl CryptoRng for RandomBytes {}

fn aes256_ctr_drbg_update(
    provided_data: Option<&GenericArray<u8, SeedLen>>,
    key: &mut GenericArray<u8, KeyLen>,
    v: &mut GenericArray<u8, BlockLen>,
) {
    let mut temp = GenericArray::<u8, SeedLen>::default();
    for i in 0..3 {
        //increment V
        for j in (0..16).rev() {
            if v[j] == 0xff {
                v[j] = 0x00;
            } else {
                v[j] += 1;
                break;
            }
        }
        let cipher = Aes256Enc::new(&key.clone().into());
        let mut block: GenericArray<u8, BlockLen> = Default::default();
        block.copy_from_slice(v);
        cipher.encrypt_block(&mut block);
        temp[i * BlockLen::USIZE..(i + 1) * BlockLen::USIZE].copy_from_slice(&block);
    }
    if let Some(data) = provided_data {
        for (t, d) in Iterator::zip(temp.iter_mut(), data.iter()) {
            *t ^= d;
        }
    }
    key.copy_from_slice(&temp[0..KeyLen::USIZE]);
    v.copy_from_slice(&temp[KeyLen::USIZE..(KeyLen::USIZE + BlockLen::USIZE)]);
}
