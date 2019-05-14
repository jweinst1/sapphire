

#[derive(Debug)]
enum SaphTree {
	Error(String),
	Stream(Vec<SaphTree>),
	Cmd(Vec<SaphTree>),
	CmdName(String),
	CmdPipe,
	Null,
	Int(i32)
}

impl SaphTree {

	pub fn parse(code:&String) -> Self {
		if code.len() == 0 {
			return SaphTree::Null
		}
		SaphTree::parse_stream(&mut code.chars().peekable())
	}

	fn parse_stream(context:&mut std::iter::Peekable<std::str::Chars>) -> Self {
		let mut arr:Vec<SaphTree> = vec![];
		loop {
			match context.peek() {
				Some(ch) => match ch {
					' ' | '\n' | '\t' => {
						context.next();
						()
					},
					'a' ... 'z' => arr.push(SaphTree::parse_cmd(context)),
					_ => return SaphTree::Error(format!("Expected 'a' ... 'z', found '{}'", context.next().unwrap()))
				},
				None => break
			}
		}
		SaphTree::Stream(arr)
	}

	fn parse_cmd(context:&mut std::iter::Peekable<std::str::Chars>) -> Self {
		SaphTree::Null
	}
}
