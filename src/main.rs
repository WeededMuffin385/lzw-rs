use bitvec::bitvec;
use bitvec::field::BitField;
use bitvec::prelude::Lsb0;
use bitvec::vec::BitVec;
use crate::lzw::*;

mod lzw;

fn main() {
    println!("[ENCODE]");
    let x = encode("TOBEORNOTTOBEORTOBEORNOT");

    println!("[DECODE]");
    let y = decode(&x);

    println!("{} | {}", x.to_string(), x.len());
    println!("{y}");
}
