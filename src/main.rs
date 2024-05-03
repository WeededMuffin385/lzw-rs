use std::fs;
use std::fs::read_to_string;
use std::mem::size_of;
use std::path::{Path, PathBuf};
use bitvec::field::BitField;
use bitvec::vec::BitVec;
use crate::lzw::*;

mod lzw;

fn test(path: impl AsRef<Path>) {
    let data = fs::read(path).unwrap();

    let bits = encode(&data);
}

fn main() {
    //test("assets/engwiki_ascii.txt");
    let input = fs::read("assets/engwiki_ascii.txt").unwrap();
    let bits = encode(&input);
    let output = decode(&bits);




    if input == output {
        println!("ok")
    } else {
        println!("err")
    };
}
