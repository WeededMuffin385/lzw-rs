use std::collections::HashMap;
use std::num::NonZeroUsize;
use std::ops::Not;
use bitvec::field::BitField;
use bitvec::prelude::{Lsb0, Msb0};
use bitvec::slice::BitSlice;
use bitvec::vec::BitVec;


pub fn encode(data: &str) -> Vec<u128> {
	let mut size: u128 = 128;
	let mut dict: HashMap<_,_> = (0..size).map(|x|((x as u8 as char).to_string(), x)).collect();

	let mut result = Vec::new();
	let mut s = data.chars().nth(0).unwrap().to_string();

	for c in data[1..data.len()].chars() {
		let w = format!("{s}{c}");

		if dict.contains_key(&w) {
			s = w;
		} else {
			result.push(dict[&s]);
			dict.insert(w, size);
			s = c.to_string();
			size += 1;
		}
	}
	result.push(dict[&s]);
	result
}


pub fn decode(data: &[u128]) -> String {
	let mut size: u128 = 128;
	let mut dict: HashMap<_,_> = (0..size).map(|x|(x, (x as u8 as char).to_string())).collect();

	let mut result = String::new();
	let mut s = String::new();

	for k in data {
		let w = dict[k].clone();
		result.push_str(&w);

		if s.is_empty().not() {
			let v = format!("{s}{}", w.chars().nth(0).unwrap());
			dict.insert(size, v);
			size += 1;
		}
		s = w;
	}
	result
}




pub fn tight_encode(data: &str) -> BitVec {
	let mut size: usize = 128;
	let mut dict: HashMap<_,_> = (0..size).map(|x|((x as u8 as char).to_string(), x)).collect();

	let mut result = BitVec::new();
	let mut s = data.chars().nth(0).unwrap().to_string();

	for c in data[1..data.len()].chars() {
		let w = format!("{s}{c}");

		if dict.contains_key(&w) {
			s = w;
		} else {
			let sc = dict[&s];
			let bits= &BitVec::<_, Lsb0>::from_element(sc)[0..(size.ilog2() + 1) as usize];
			result.extend(bits);

			dict.insert(w, size);
			s = c.to_string();
			size += 1;
		}
	}
	let sc = dict[&s];
	let bits= &BitVec::<_, Lsb0>::from_element(sc)[0..(size.ilog2() + 1) as usize];
	result.extend(bits);

	result
}

pub fn tight_decode(data: &BitSlice) -> String {
	let mut size: usize = 128;
	let mut dict: HashMap<_,_> = (0..size).map(|x|(x, (x as u8 as char).to_string())).collect();

	let mut result = String::new();
	let mut s = String::new();

	let mut index: usize = 0;
	loop {
		let length = size.ilog2() as usize + 1;
		let bits = &data[index..index + length];
		let k = bits.load();
		let w = dict[&k].clone();
		result.push_str(&w);

		index += length;

		if s.is_empty().not() {
			let v = format!("{s}{}", w.chars().nth(0).unwrap());
			dict.insert(size, v);
			size += 1;
		}
		s = w;

		if index == data.len() {break}
	}
	result
}