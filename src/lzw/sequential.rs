use std::mem::size_of;
use bitvec::field::BitField;
use bitvec::prelude::BitSlice;
use bitvec::vec::BitVec;
use crate::lzw::{decode, encode};

const USIZE_BIT_SIZE: usize = size_of::<usize>() * 8;


pub fn encode_sequential(data: &[u8]) -> BitVec {
	let data = encode(data);
	let mut result = BitVec::from_element(data.len());
	result.extend_from_bitslice(&data);
	result
}


pub fn decode_sequential(mut data: &BitSlice) -> Vec<u8> {
	let (len, slice) = data.split_at(USIZE_BIT_SIZE);
	let (data, _) = slice.split_at(len.load());
	decode(data)
}