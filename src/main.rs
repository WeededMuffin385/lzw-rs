use std::fs;
use std::fs::read_to_string;
use std::mem::size_of;
use bitvec::bitvec;
use bitvec::field::BitField;
use bitvec::prelude::Lsb0;
use bitvec::vec::BitVec;
use crate::lzw::*;

mod lzw;

fn main() {
    {
        let data = read_to_string("assets/engwiki_ascii.txt").unwrap();
        println!("[ENCODE]");
        let x = encode(&data);

        let mut data = Vec::new();
        for value in x.into_vec() {
            data.extend(value.to_le_bytes());
        }
        fs::write("assets/encoded.bin", data).unwrap();
    }


    {
        let mut data = Vec::new();
        for value in fs::read("assets/encoded.bin").unwrap().chunks(size_of::<usize>()) {
            data.push(usize::from_le_bytes(value.try_into().unwrap()));
        }

        let data = BitVec::from_vec(data);
        let data = decode(&data);
        println!("{data}");
    }
}
