

#[cfg(test)]
mod parser_e2e_tests {
	use sapphire::parse::SaphTree;
	#[test]
	fn empty_stream_test() {
		let code = String::from("[]");
		match SaphTree::parse(&code) {
			SaphTree::Stream(nodes) => assert!(nodes.len() == 0),
			SaphTree::Error(e) => panic!("test failed got error {}", e),
			_ => panic!("Test for empty stream parsing failed")
		}
	}
}
