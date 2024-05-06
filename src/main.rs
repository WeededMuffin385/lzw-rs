use std::fs;
use std::mem::size_of;
use std::path::Path;
use bitvec::prelude::*;
use crate::lzw::parallel::{decode_parallel, encode_parallel};
use crate::lzw::sequential::{decode_sequential, encode_sequential};

mod lzw;

fn test_sequential(path: impl AsRef<Path>) -> bool {
    {
        println!("[ENCODING]");
        let input = fs::read(path.as_ref()).unwrap();
        let bits = encode_sequential(&input);
        let data = bits.into_vec();
        let mut result = Vec::with_capacity(data.len() * size_of::<usize>());
        for value in data { result.extend_from_slice(&value.to_le_bytes()); }
        fs::write(path.as_ref().with_extension("slzw"), result.clone()).unwrap();
    }



    let result_path = path.as_ref().with_file_name(format!("{}_sequential_result", path.as_ref().file_stem().unwrap().to_str().unwrap())).with_extension("txt");
    {
        println!("[DECODING]");
        let input = fs::read(path.as_ref().with_extension("slzw")).unwrap();
        let mut data = Vec::new();
        for value in input.chunks_exact(size_of::<usize>()) { data.push(usize::from_le_bytes(value.try_into().unwrap())); }
        let data = BitVec::from_vec(data);
        let result = decode_sequential(&data);

        fs::write(result_path.clone(), result.clone()).unwrap();
    }

    let original = fs::read(path.as_ref().clone()).unwrap();
    let result = fs::read(result_path.clone()).unwrap();

    original == result
}

fn test_parallel(path: impl AsRef<Path>) -> bool {
    {
        println!("[ENCODING]");
        let input = fs::read(path.as_ref()).unwrap();
        let bits = encode_parallel(&input, 2usize.pow(20) * 48);
        let data = bits.into_vec();
        let mut result = Vec::with_capacity(data.len() * size_of::<usize>());
        for value in data { result.extend_from_slice(&value.to_le_bytes()); }
        fs::write(path.as_ref().with_extension("plzw"), result.clone()).unwrap();
    }



    let result_path = path.as_ref().with_file_name(format!("{}_parallel_result", path.as_ref().file_stem().unwrap().to_str().unwrap())).with_extension("txt");
    {
        println!("[DECODING]");
        let input = fs::read(path.as_ref().with_extension("plzw")).unwrap();
        let mut data = Vec::new();
        for value in input.chunks_exact(size_of::<usize>()) { data.push(usize::from_le_bytes(value.try_into().unwrap())); }
        let data = BitVec::from_vec(data);
        let result = decode_parallel(&data);
        fs::write(result_path.clone(), result.clone()).unwrap();
    }

    let original = fs::read(path.as_ref().clone()).unwrap();
    let result = fs::read(result_path.clone()).unwrap();

    original == result
}

fn test(path: impl AsRef<Path>) {
    let parallel_result = test_parallel(&path);
    let sequential_result = test_sequential(&path);

    if parallel_result {
        println!("parallel ok");
    } else {
        println!("parallel err");
    }

    if sequential_result {
        println!("sequential ok");
    } else {
        println!("sequential err");
    }
}

fn main() {
    test("assets/engwiki_ascii.txt");
}
