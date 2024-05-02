use crate::lzw::{decode, encode, tight_decode, tight_encode};

mod lzw;

fn main() {
    let x = tight_encode("TOBEORNOTTOBEORTOBEORNOT");
    let y = tight_decode(&x);

    println!("{} | {}", x.to_string(), x.len());
    println!("{y}");
}
