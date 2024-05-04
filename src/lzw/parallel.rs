use std::collections::HashSet;
use std::mem::size_of;
use std::ops::Not;
use std::sync::mpsc::channel;
use bitvec::field::BitField;
use bitvec::order::Lsb0;
use bitvec::prelude::{BitSlice, BitVec};
use rayon::ThreadPoolBuilder;
use crate::lzw::{decode, encode};

const CHUNK_SIZE_MEGABYTES: usize = 64;
const USIZE_BIT_SIZE: usize = size_of::<usize>() * 8;

pub fn encode_parallel(data: &[u8]) -> BitVec {
	let pool = ThreadPoolBuilder::new().num_threads(8).build().unwrap();
	let (sender, receiver) = channel();

	let mut len = 0;

	pool.scope(|scope|{
		for (index, data) in data.chunks(2usize.pow(20) * CHUNK_SIZE_MEGABYTES).enumerate() {
			len += 1;
			let sender = sender.clone();

			scope.spawn(move |_| {
				sender.send((encode(data), index)).unwrap();
			});
		}
		println!("chunks: {len}");
	});


	let mut set: HashSet<_> = (0..len).collect();
	let mut results = vec![Default::default(); len];

	while set.is_empty().not() {
		if let Ok((data, index)) = receiver.try_recv() {
			results[index] = data;
			set.remove(&index);
		}
	}

	let mut result = BitVec::new();
	for value in results {
		result.extend_from_bitslice(&BitVec::<usize, Lsb0>::from_element(value.len()));
		result.extend_from_bitslice(&value);
	}

	result
}



pub fn decode_parallel(mut data: &BitSlice) -> Vec<u8> {
	let mut chunks = Vec::new();
	while data.len() >= USIZE_BIT_SIZE {
		let len = data[0..USIZE_BIT_SIZE].load_le::<usize>() as usize;
		data = &data[USIZE_BIT_SIZE..data.len()];
		chunks.push(data[0..len].to_bitvec());
		data = &data[len..data.len()];
	}

	let pool = ThreadPoolBuilder::new().num_threads(8).build().unwrap();
	let (sender, receiver) = channel();

	pool.scope(|scope|{
		for (index, data) in chunks.iter().enumerate() {
			let sender = sender.clone();

			scope.spawn(move |_| {
				sender.send((decode(data), index)).unwrap();
			});
		}
	});


	let len = chunks.len();
	let mut set: HashSet<_> = (0..len).collect();
	let mut results = vec![Default::default(); len];

	while set.is_empty().not() {
		if let Ok((data, index)) = receiver.try_recv() {
			results[index] = data;
			set.remove(&index);
		}
	}

	results.concat()
}