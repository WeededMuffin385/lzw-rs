use std::collections::HashMap;
use std::fmt::format;
use std::io;
use std::io::Write;
use std::num::NonZeroUsize;
use std::ops::Not;
use bitvec::bitvec;
use bitvec::field::BitField;
use bitvec::prelude::{Lsb0, Msb0};
use bitvec::slice::BitSlice;
use bitvec::vec::BitVec;


fn get_len(value: usize) -> usize {
	let log = value.ilog2();
	let add = if 2usize.pow(log) != value {1} else {0};
	(log + add) as usize
}


pub fn encode(data: &str) -> BitVec {
	let mut dict: HashMap<_,_> = (0..128u8).map(|x|((x as char).to_string(), x as usize)).collect();

	let mut result = BitVec::new();
	let mut s = data.chars().nth(0).unwrap().to_string();

	for (index, c) in data[1..data.len()].chars().enumerate() {
		let w = format!("{s}{c}");

		if dict.contains_key(&w) {
			s = w;
		} else {
			let sc = dict[&s];
			let len = get_len(dict.len());
			let bits= &BitVec::<_, Lsb0>::from_element(sc)[0..len];
			result.extend_from_bitslice(bits);

			// println!("{len}|{sc}|{s}|{bits}");


			dict.insert(w, dict.len());
			s = c.to_string();
		}


		if index % 1_000_000 == 0 { println!("{:.6}%", (index as f32) / (data.len() as f32) * 100.0) };
	}
	let sc = dict[&s];
	let len = get_len(dict.len());
	let bits= &BitVec::<_, Lsb0>::from_element(sc)[0..len];
	result.extend_from_bitslice(bits);
	result.extend_from_bitslice(&bitvec![0;len]);

	// result.extend_from_bitslice()

	result
}

pub fn decode(mut data: &BitSlice) -> String {
	let mut dict: HashMap<_,_> = (0..128u8).map(|x|(x as usize, (x as char).to_string())).collect();

	let mut result = String::new();
	let mut s = String::new();

	let length = data.len();

	loop {
		let len = get_len(dict.len());
		let bits = &data[0..len];
		let k = bits.load_le();

		if k == 0 {break}

		if dict.contains_key(&k) {
			let w = dict[&k].clone();
			result.push_str(&w);

			if s.is_empty().not() {
				let v = format!("{s}{}", w.chars().nth(0).unwrap());
				dict.insert(dict.len(), v);
			}

			s = w;
		} else {
			let v = format!("{s}{}", s.chars().nth(0).unwrap());
			result.push_str(&v);
			dict.insert(dict.len(), v);
		}


		if dict.len() % 1_000_000 == 0 {println!("{:.6}%", (length - data.len()) as f32 / (length as f32) * 100.0)}
		data = &data[get_len(dict.len())..];
	}
	result
}