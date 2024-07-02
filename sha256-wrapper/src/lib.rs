#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![feature(test)]
extern crate test;


use std::{mem::MaybeUninit, usize};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn sha256hash(string: &str) -> Vec<u8> {
    let mut sha_ctx = MaybeUninit::<*mut EVP_MD_CTX>::uninit();
    // let mut buf = [0u8; EVP_MAX_MD_SIZE as usize];
    let mut buf = vec![0u8; EVP_MAX_MD_SIZE as usize];
    let mut md_len : u32 = EVP_MAX_MD_SIZE;
    unsafe {
        sha_ctx.write(EVP_MD_CTX_new());
        EVP_DigestInit_ex(sha_ctx.assume_init(), EVP_sha256(), std::ptr::null_mut());
        EVP_DigestUpdate(
            sha_ctx.assume_init(),
            string.as_ptr() as *mut _,
            string.len() as usize,
        );
        EVP_DigestFinal_ex(sha_ctx.assume_init(), buf.as_mut_ptr(), (&mut md_len) as (*mut u32));
    }
    buf.truncate(md_len as usize);
    buf
}

#[cfg(test)]
mod tests {
    extern crate hex;
    use sha2::{Sha256, Sha512, Digest};
    use super::*;
    use test::Bencher;

    #[test]
    fn sha256hash_check() {
        let long_string = "Decidable-Unsavory-Marmalade-Onward-Bazooka-Supply-Hardness-Boondocks-Cosmic-Improving";
        let correct_hash =
            hex::decode("872fa4d06aeac5798bd7a1412e32786196fae598bf7dbc1667f1f65a0f6cb4e6")
                .expect("Decoding failed");
        assert_eq!(sha256hash(long_string), &correct_hash[..]);
    }
    #[test]
    fn sha256hash_check2() {
        let long_string = r#"{
 "location": [
  "1515 Extension Rd",
  "Mesa",
  "AZ",
  "85210"
 ]
}"#;
        let correct_hash =
            hex::decode("6a47d2779c21a4b42ab896a1ae56818a720e6b2be698d61953759a5eb3fde994")
                .expect("Decoding failed");
        assert_eq!(sha256hash(long_string), &correct_hash[..]);
    }

    #[bench]
    fn bench_sha_256(b: &mut Bencher){
        let string = "Decidable-Unsavory-Marmalade-Onward-Bazooka-Supply-Hardness-Boondocks-Cosmic-Improving";
        b.iter(|| sha256hash(string))
    }

    #[bench]
    fn bench_native_sha256(b: &mut Bencher){
        let string = "Decidable-Unsavory-Marmalade-Onward-Bazooka-Supply-Hardness-Boondocks-Cosmic-Improving";
        b.iter(|| {
            // create a Sha256 object
            let mut hasher = Sha256::new();

            // write input message
            hasher.update(string);

            // read hash digest and consume hasher
            let result = hasher.finalize();
        })
    }
}

