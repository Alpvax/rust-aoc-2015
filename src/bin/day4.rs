extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;


fn main() {
  let input = "ckczppom".as_bytes();
  let mut i = 0;
  let mut hasher = Md5::new();
  loop {
    i += 1;
    hasher.input(input);
    hasher.input(i.to_string().as_bytes());

    let mut hash = [0; 16]; // An MD5 is 16 bytes
    hasher.result(&mut hash);
    if hash[..3] == [0, 0, 0] {// && hash[2] <= 0x0F { // First 5 chars = first 2.5 bytes
      println!("{}: {:?}", i, hash);
      break;
    }
    hasher.reset();
  }
}