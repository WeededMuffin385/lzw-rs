use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::num::NonZeroUsize;
use std::ops::Not;
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

	for c in data[1..data.len()].chars() {
		let w = format!("{s}{c}");

		if dict.contains_key(&w) {
			s = w;
		} else {
			let sc = dict[&s];
			let len = get_len(dict.len());
			let bits= &BitVec::<_, Lsb0>::from_element(sc)[0..len];
			result.extend(bits);

			//println!("{len}|{sc}|{s}|{bits}");


			dict.insert(w, dict.len());
			s = c.to_string();
		}
	}
	let sc = dict[&s];
	let len = get_len(dict.len());
	let bits= &BitVec::<_, Lsb0>::from_element(sc)[0..len];
	result.extend(bits);

	//println!("{len}|{sc}|{s}|{bits}");

	result
}

pub fn decode(mut data: &BitSlice) -> String {
	let mut dict: HashMap<_,_> = (0..128u8).map(|x|(x as usize, (x as char).to_string())).collect();

	let mut result = String::new();
	let mut s = String::new();

	loop {
		let len = get_len(dict.len());
		let bits = &data[0..len];
		let k = bits.load_le();
		//print!("{len}|{k}|");
		io::stdout().flush().unwrap();
		let w = dict[&k].clone();
		//println!("{w}");
		result.push_str(&w);

		if s.is_empty().not() {
			let v = format!("{s}{}", w.chars().nth(0).unwrap());
			dict.insert(dict.len(), v);
		}
		s = w;

		data = &data[get_len(dict.len())..];
		if data.is_empty() {break}
	}
	result
}