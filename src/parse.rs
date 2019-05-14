

#[derive(Debug)]
enum SaphTree {
	Error(String),
	Stream(Vec<SaphTree>),
	Cmd(Vec<SaphTree>),
	CmdName(String),
	CmdPipe,
	Null,
	Number(f32)
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
					'a' ... 'z' => {
						let cmd_got = SaphTree::parse_cmd(context);
						match cmd_got {
							SaphTree::Error(_) => return cmd_got,
							_ => arr.push(cmd_got)
						}
					},
					'|' | '>' => {
						context.next();
						arr.push(SaphTree::CmdPipe);
					}
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
		// Command arguments
		loop {
			match context.peek() {
				Some(ch) => match ch {
					// pipes and arrows are ends of a command
					'|' => break,
					'-' => {
						context.next();
						match context.peek() {
								Some(ch) => match ch {
								'0' ... '9' =>  {
									let num_got = SaphTree::parse_number(context, false);
									match num_got {
										SaphTree::Error(_) => return num_got,
										_ => arr.push(num_got)
									}	
								},
								'>' => break,
								_ => return SaphTree::Error(format!("Expected '0' ... '9' or ->, got '-{}'", context.next().unwrap()))
							},
							None => return SaphTree::Error(String::from("Expected '0' ... '9' or ->, got end of input."))
						}
					},
					'0' ... '9' => {
						let num_got = SaphTree::parse_number(context, true);
						match num_got {
							SaphTree::Error(_) => return num_got,
							_ => arr.push(num_got)
						}
					},
					_ => return SaphTree::Error(format!("Expected argument, found '{}'", context.next().unwrap()))
				},
				None => break
			}
		}
		SaphTree::Cmd(arr)
	}

	fn parse_cmd_name(context:&mut std::iter::Peekable<std::str::Chars>) -> Self {
		let mut name = String::new();
		loop {
			match context.next() {
				Some(ch) => match ch {
					'a' ... 'z' => name.push(ch),
					' ' | '\n' | '\t' => break,
					_ => return SaphTree::Error(format!("Expected cmd_name: 'a' ... 'z', found '{}'", context.next().unwrap()))
				},
				None => break
			}
		}
		SaphTree::CmdName(name)
	}

	fn parse_number(context:&mut std::iter::Peekable<std::str::Chars>, pos:bool) -> Self {
		let mut num = String::new();
		loop {
			match context.peek() {
				Some(ch) => match ch {
					' ' | '\n' | '\t' => break,
					'0' ... '9' => num.push(context.next().unwrap()),
					'.' => num.push(context.next().unwrap()),
					')' => break,
					_ => return SaphTree::Error(format!("Expected number: '0' ... '9', found '{}'", context.next().unwrap()))
				},
				None => break
			}
		}
		match num.parse::<f32>() {
			Ok(pnf) => {
				if pos {
					return SaphTree::Number(num.parse::<f32>().unwrap())
				} else {
					return SaphTree::Number(-(num.parse::<f32>().unwrap()))
				}
			},
			Err(_) => return SaphTree::Error(format!("Found invalid number literal: '{}'", num))
		}
	}
}
