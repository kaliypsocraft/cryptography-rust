// Using std library/crate and io module
use std::io;
use crate::crypto::{Cipher, Vigenere};
pub mod linear;
pub mod crypto;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line!"); 
    let mut test = Vigenere {
        plain_text : input,
    };
    let encrypted_text = test.encrypt();
    println!("Encrypted: {}", encrypted_text);
    test.plain_text = encrypted_text;
    let decrypted_text = test.decrypt();
    println!("Decrypted: {}", decrypted_text);
    // println!("Decrypted: {}", rot13(&encrypted));

   
}



/*
#[warn(dead_code)]
fn rot13(plain_text : &String) -> String{
    let mut encrypted_str = String::new();
    for letter in plain_text.to_ascii_uppercase().bytes() {
        if is_letter(letter) {
            let encrypted_letter = ((letter + ROT_13 - OFFSET) % ALPHABET_MOD) + OFFSET;
            encrypted_str.push(encrypted_letter as char);
        } else {
            encrypted_str.push(letter as char)
        }
    }
    encrypted_str
}
*/

