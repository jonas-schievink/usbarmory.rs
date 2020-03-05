//! Sanity check the AES-128 API when paired with the unreadable UNIQUE key

#![allow(dead_code)]
#![deny(unused_must_use)]
#![no_main]
#![no_std]

use block_cipher_trait::BlockCipher as _;
use exception_reset as _; // default exception handler
use panic_serial as _; // panic handler
use usbarmory::{dcp::Aes128, memlog, memlog_flush_and_reset};

// NOTE binary interfaces, using `no_mangle` and `extern`, are extremely unsafe
// as no type checking is performed by the compiler; stick to safe interfaces
// like `#[rtfm::app]`
#[no_mangle]
fn main() -> ! {
    let cipher = Aes128::new_unique();

    let plaintext = [
        179, 176, 19, 230, 198, 237, 169, 162, 83, 237, 103, 21, 175, 240, 64, 242,
    ];

    memlog!("plaintext:  {:?}", plaintext);

    // plaintext encrypted with an all-zeros key
    let wrong = [
        199, 166, 244, 229, 161, 134, 82, 116, 3, 104, 116, 10, 198, 165, 146, 116,
    ]
    .into();
    let mut block = plaintext.into();

    cipher.encrypt_block(&mut block);

    memlog!("ciphertext: {:?}", block);
    memlog!("wrong:      {:?}", wrong);

    if block != wrong {
        memlog!("encrypt: OK");
    } else {
        memlog!("error: UNIQUE key is all zeros!");
        memlog_flush_and_reset!();
    }

    cipher.decrypt_block(&mut block);

    if block[..] == plaintext[..] {
        memlog!("decrypt: OK");
    } else {
        memlog!("error: decryption didn't return the input plaintext");
        memlog_flush_and_reset!();
    }

    memlog_flush_and_reset!();
}
