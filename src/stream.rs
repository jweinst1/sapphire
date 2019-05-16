use crate::parse::*;
use crate::object::*;


struct SaphStream {
	items:Vec<SaphObject>
}

impl SaphStream {
	fn new(size:usize) -> Self {
		SaphStream {items:Vec::with_capacity(size)}
	}
}
