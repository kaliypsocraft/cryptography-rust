
const ALPHABET_MOD : u8 = 26;
// sconst ROT_13 : u8 = 13;
const OFFSET : u8 = 65;

pub trait Cipher {
    fn encrypt(&self) -> String;
    fn decrypt(&self) -> String;
}

pub struct Vigenere {
    pub plain_text : String,
}
pub struct Caesar {
    shift : u8,
    plain_text : String,
}


impl Cipher for Vigenere  {
    fn encrypt(&self) -> String {
        let plain_text = &self.plain_text;
        println!("Plain Text: {}", plain_text);
        let key = String::from("KEY");
        let mut encrypted_str = String::new();
        let padded_key = padding(plain_text.len(), &key);
        let padded_key = padded_key.as_bytes();
    
        let mut index = 0;
        for letter in plain_text.to_ascii_uppercase().bytes() {
            println!("Letter: {}", letter);
            if 65 <= letter && letter <= 90  {
                let encrypted_letter = ((letter + padded_key[index] - OFFSET) % ALPHABET_MOD) + OFFSET;
                encrypted_str.push(encrypted_letter as char);
            } else {
                encrypted_str.push(letter as char);
            }
            index += 1;
        }
        encrypted_str
    }
    fn decrypt(&self) -> String {
        let encrypted_text = &self.plain_text;
        let key = String::from("KEY");
        let mut decrypted_str = String::new();
        let padded_key = padding(encrypted_text.len(), &key);
        let padded_key = padded_key.as_bytes();
    
        let mut index = 0;
        for letter in encrypted_text.bytes() {
            if 65 <= letter && letter <= 90 {
                let encrypted_letter = (letter + OFFSET - padded_key[index]) % ALPHABET_MOD + OFFSET;
                decrypted_str.push(encrypted_letter as char);
            } else {
                decrypted_str.push(letter as char);
            }
            index += 1;
        }
        decrypted_str
    }
}

#[warn(dead_code)]

impl Cipher for Caesar {
    fn encrypt(&self) -> String {
        let mut encrypted_str = String::new();
        let plain_text = &self.plain_text;
        let shift = self.shift;
        for letter in plain_text.to_ascii_uppercase().bytes() {
            if 65 <= letter && letter <= 90 {
                let encrypted_letter = ((letter + shift - OFFSET) % ALPHABET_MOD) + OFFSET;
                encrypted_str.push(encrypted_letter as char);
            } else {
                encrypted_str.push(letter as char)
            }
        }
        encrypted_str
    }
    fn decrypt(&self) -> String {
        let mut encrypted_str = String::new();
        let plain_text = &self.plain_text;
        let shift = self.shift;
        for letter in plain_text.to_ascii_uppercase().bytes() {
            if 65 <= letter && letter <= 90 {
                let encrypted_letter = ((letter + shift - OFFSET) % ALPHABET_MOD) + OFFSET;
                encrypted_str.push(encrypted_letter as char);
            } else {
                encrypted_str.push(letter as char)
            }
        }
        encrypted_str
    }
}

fn padding(length :usize , key : &String) -> String {
    let mut padded_str = String::new();
    while padded_str.len() < length {
        padded_str.push_str(&key)
    }
    padded_str[0..length].to_string()
}
