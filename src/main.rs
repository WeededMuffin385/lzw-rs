use std::fs;
use std::fs::read_to_string;
use std::mem::size_of;
use std::path::{Path, PathBuf};
use bitvec::field::BitField;
use bitvec::order::Lsb0;
use bitvec::vec::BitVec;
use crate::lzw::*;

mod lzw;

fn test(path: impl AsRef<Path>) -> bool {
    {
        println!("[ENCODING]");
        let input = fs::read(path.as_ref()).unwrap();
        let bits = encode(&input);
        let data = bits.into_vec();
        let mut result = Vec::with_capacity(data.len() * size_of::<usize>());
        for value in data { result.extend_from_slice(&value.to_le_bytes()); }
        fs::write(path.as_ref().with_extension("lzw"), result.clone()).unwrap();
    }


    let result_path = path.as_ref().with_file_name(format!("result_{}", path.as_ref().file_name().unwrap().to_str().unwrap()));
    {
        println!("[DECODING]");
        let input = fs::read(path.as_ref().with_extension("lzw")).unwrap();
        let mut data = Vec::new();
        for value in input.chunks_exact(size_of::<usize>()) { data.push(usize::from_le_bytes(value.try_into().unwrap())); }
        let data = BitVec::from_vec(data);
        let result = decode(&data);

        fs::write(result_path.clone(), result.clone()).unwrap();
    }

    let original = fs::read(path.as_ref().clone()).unwrap();
    let result = fs::read(result_path.clone()).unwrap();

    original == result
}

fn main() {
    if test("assets/engwiki_ascii.txt") {
        println!("ok")
    } else {
        println!("err")
    };
}
