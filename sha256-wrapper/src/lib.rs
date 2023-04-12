#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::mem::MaybeUninit;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn sha256hash(string: &str) -> [u8; SHA256_DIGEST_LENGTH as usize] {
    let mut sha_ctx = MaybeUninit::<SHA256_CTX>::uninit();
    let mut buf = [0u8; SHA256_DIGEST_LENGTH as usize];
    unsafe {
        SHA256_Init(sha_ctx.as_mut_ptr());
        SHA256_Update(
            sha_ctx.as_mut_ptr(),
            string.as_ptr() as *mut _,
            string.len() as usize,
        );
        SHA256_Final(buf.as_mut_ptr(), sha_ctx.as_mut_ptr());
    }
    buf
}

#[cfg(test)]
mod tests {
    extern crate hex;

    use super::*;

    #[test]
    fn sanity_check_for_generated_code() {
        unsafe {
            /*
            Based on: https://www.py4u.net/discuss/64701
            */
            let mut sha_ctx = MaybeUninit::<SHA256_CTX>::uninit();
            SHA256_Init(sha_ctx.as_mut_ptr());
            let long_string = "Decidable-Unsavory-Marmalade-Onward-Bazooka-Supply-Hardness-Boondocks-Cosmic-Improving";
            SHA256_Update(
                sha_ctx.as_mut_ptr(),
                long_string.as_ptr() as *mut _,
                long_string.len() as usize,
            );
            let mut buf = [0u8; SHA256_DIGEST_LENGTH as usize];
            SHA256_Final(buf.as_mut_ptr(), sha_ctx.as_mut_ptr());
            let correct_hash =
                hex::decode("872fa4d06aeac5798bd7a1412e32786196fae598bf7dbc1667f1f65a0f6cb4e6")
                    .expect("Decoding failed");
            assert_eq!(buf, &correct_hash[..]);
        }
    }

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
}
