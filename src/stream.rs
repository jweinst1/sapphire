use crate::parse::*;
use crate::object::*;

pub enum SaphCurrentBin {
	First,
	Second
}


pub struct SaphStream {
	bin_1:Vec<SaphObject>,
	bin_2:Vec<SaphObject>,
	state:SaphCurrentBin
}

impl SaphStream {
	pub fn new(size:usize) -> Self {
		SaphStream {
			bin_1:Vec::with_capacity(size),
			bin_2:Vec::with_capacity(size),
			state:SaphCurrentBin::First
		}
	}
}

impl SaphStream {
	pub fn get_items(&self) -> &Vec<SaphObject> {
		match self.state {
			SaphCurrentBin::First => &self.bin_1,
			SaphCurrentBin::Second => &self.bin_2
		}
	}
}
