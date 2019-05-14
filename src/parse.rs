

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
					_ => return SaphTree::Error(format!("Expected cmd: 'a' ... 'z', found '{}'", context.next().unwrap()))
				},
				None => break
			}
		}
		SaphTree::Stream(arr)
	}

	fn parse_cmd(context:&mut std::iter::Peekable<std::str::Chars>) -> Self {
		let mut arr:Vec<SaphTree> = vec![];
		let name = SaphTree::parse_cmd_name(context);
		// Check if name found correctly or not.
		match name {
			SaphTree::Error(_) => return name,
			_ => arr.push(name)
		}
		/*loop {
			match context.peek() {

			}
		}*/
		SaphTree::Cmd(arr)
	}

	fn parse_cmd_name(context:&mut std::iter::Peekable<std::str::Chars>) -> Self {
		let mut name = String::new();
		loop {
			match context.next() {
				Some(ch) => match ch {
					'a' ... 'z' => name.push(ch),
					' ' | '\n' | '\t' => (),
					_ => return SaphTree::Error(format!("Expected cmd_name: 'a' ... 'z', found '{}'", context.next().unwrap()))
				},
				None => break
			}
		}
		SaphTree::CmdName(name)
	}
}
