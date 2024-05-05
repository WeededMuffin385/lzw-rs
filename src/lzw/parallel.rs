use std::mem::size_of;
use bitvec::field::BitField;
use bitvec::order::Lsb0;
use bitvec::prelude::{BitSlice, BitVec};
use rayon::ThreadPoolBuilder;
use crate::lzw::{decode, encode};

const CHUNK_SIZE_MEGABYTES: usize = 48;
const USIZE_BIT_SIZE: usize = size_of::<usize>() * 8;

pub fn encode_parallel(data: &[u8]) -> BitVec {
	let pool = ThreadPoolBuilder::new().num_threads(8).build().unwrap();
	let mut data: Vec<_> = data.chunks(2usize.pow(20) * CHUNK_SIZE_MEGABYTES).map(|data|(data, BitVec::default())).collect();

	pool.scope(|scope|{
		for (data, result) in &mut data {
			scope.spawn(|_| {
				*result = encode(data);
			});
		}
	});


	let mut result = BitVec::new();
	for (_,value) in data {
		result.extend_from_bitslice(&BitVec::<usize, Lsb0>::from_element(value.len()));
		result.extend_from_bitslice(&value);
	}

	result
}



pub fn decode_parallel(mut data: &BitSlice) -> Vec<u8> {
	let pool = ThreadPoolBuilder::new().num_threads(8).build().unwrap();

	let mut chunks = Vec::new();
	while data.len() >= USIZE_BIT_SIZE {
		let len = data[0..USIZE_BIT_SIZE].load::<usize>();
		data = &data[USIZE_BIT_SIZE..data.len()];
		chunks.push(&data[0..len]);
		data = &data[len..data.len()];
	}

	let mut data: Vec<_> = chunks.iter().map(|data|(data, Vec::default())).collect();

	pool.scope(|scope|{
		for (data, result) in &mut data {
			scope.spawn(move |_| {
				*result = decode(data);
			});
		}
	});

	data.iter().flat_map(|(_, value)|value.clone()).collect()
}