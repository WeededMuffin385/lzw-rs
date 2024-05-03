use std::collections::HashMap;
use bitvec::bitvec;
use bitvec::field::BitField;
use bitvec::prelude::Lsb0;
use bitvec::slice::BitSlice;
use bitvec::vec::BitVec;


fn get_len(value: usize) -> usize {
	let log = value.ilog2();
	let add = if 2usize.pow(log) != value {1} else {0};
	(log + add) as usize
}


pub fn encode(data: &[u8]) -> BitVec {
	let mut dict: HashMap<_,_> = (0..=255u8).map(|x|(vec![x], x as usize)).collect();

	let mut result = BitVec::new();
	let mut w = Vec::new();

	for (index, &c) in data.iter().enumerate() {
		let wc = vec![w.clone(), vec![c]].concat();

		if dict.contains_key(&wc) {
			w = wc;
		} else {
			let len = get_len(dict.len());
			let bits= &BitVec::<_, Lsb0>::from_element(dict[&w])[0..len];
			result.extend_from_bitslice(bits);

			//println!("{len}|{:<3}|{}|{bits}", String::from_utf8_lossy(&w), dict[&w]);

			dict.insert(wc, dict.len());
			w.clear();
			w.push(c);
		}
		if index % 1_000_000 == 0 { println!("{:.6}%", (index as f32) / (data.len() as f32) * 100.0) };
	}

	let len = get_len(dict.len());
	let bits= &BitVec::<_, Lsb0>::from_element(dict[&w])[0..len];
	result.extend_from_bitslice(bits);
	//println!("{len}|{:<3}|{}|{bits}", String::from_utf8_lossy(&w), dict[&w]);

	result.extend_from_bitslice(&bitvec![0;len]);
	result
}


pub fn decode(mut data: &BitSlice) -> Vec<u8> {
	let mut dict: HashMap<_,_> = (0..=255u8).map(|x|(x as usize, vec![x])).collect();
	let length = data.len();

	fn get(data: &BitSlice, dict: &mut HashMap<usize, Vec<u8>>) -> usize {
		let len = get_len(dict.len());
		let bits = &data[0..len];
		bits.load_le()
	}

	let k = get(&mut data, &mut dict);
	let mut w = dict[&k].clone();
	let mut result = w.clone();


	data = &data[get_len(dict.len())..];
	//println!("{}|{}", get_len(dict.len()), k);

	loop {
		let k = get(&mut data, &mut dict);
		//println!("{}|{}", get_len(dict.len()), k);
		if k == 0 {break}

		let entry = if dict.contains_key(&k) {
			dict[&k].clone()
		} else if k == dict.len() {
			let mut entry = w.clone();
			entry.push(w[0]);
			entry
		} else {
			panic!("Invalid dictionary!");
		};

		result.extend_from_slice(&entry);
		w.push(entry[0]);
		dict.insert(dict.len(), w);

		data = &data[get_len(dict.len())..];
		w = entry;

		if dict.len() % 1_000_000 == 0 {println!("{:.6}%", (length - data.len()) as f32 / (length as f32) * 100.0)}
	}
	result
}