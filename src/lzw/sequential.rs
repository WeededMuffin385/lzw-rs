use std::mem::size_of;
use bitvec::field::BitField;
use bitvec::prelude::BitSlice;
use bitvec::vec::BitVec;
use crate::lzw::{decode, encode};

const USIZE_BIT_SIZE: usize = size_of::<usize>() * 8;


pub fn encode_sequential(data: &[u8]) -> BitVec {
	let bits = encode(data);
	let mut result = BitVec::from_element(bits.len());
	result.extend_from_bitslice(&bits);
	result
}


pub fn decode_sequential(mut data: &BitSlice) -> Vec<u8> {
	let len = data[0..USIZE_BIT_SIZE].load_le();
	let data = &data[USIZE_BIT_SIZE..data.len()];
	decode(&data[0..len])
}