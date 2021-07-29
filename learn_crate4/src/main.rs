extern crate crypto;


use cryto::digest::Digest;
use cryto::sha3::Sha3;

fn main() {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str("hello world");
    let result = hasher.result_str();
    println!("hash = {}", result);

    println!("Hello, world!");
}
