use std::mem::size_of;
use bitvec::field::BitField;
use bitvec::order::Lsb0;
use bitvec::prelude::{BitSlice, BitVec};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::{IndexedParallelIterator, ParallelSlice};
use crate::lzw::{decode, encode};

const CHUNK_SIZE_MEGABYTES: usize = 48;
const USIZE_BIT_SIZE: usize = size_of::<usize>() * 8;

pub fn encode_parallel(data: &[u8]) -> BitVec {
	let data: Vec<_> = data.par_chunks(2usize.pow(20) * CHUNK_SIZE_MEGABYTES).map(|data|encode(data)).collect();

	let mut result = BitVec::new();
	for value in data {
		result.extend_from_bitslice(&BitVec::<usize, Lsb0>::from_element(value.len()));
		result.extend_from_bitslice(&value);
	}

	result
}



pub fn decode_parallel(mut data: &BitSlice) -> Vec<u8> {
	let mut chunks = Vec::new();

	while data.len() >= USIZE_BIT_SIZE {
		let (len, slice) = data.split_at(USIZE_BIT_SIZE);
		let len = len.load();
		let (chunk, slice) = slice.split_at(len);
		chunks.push(chunk);
		data = slice;
	}

	chunks.par_iter().flat_map(|data|decode(data)).collect()
}