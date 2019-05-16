
/// The `SaphTree` enum represents the abstract syntax tree
/// used by Sapphire to parse through in coming stream text.
/// ## Usage
/// The `SaphTree` can undertake a variety of forms, each of which has different typical
/// numbers of child nodes. This enum can be traversed like a tree, with the `Stream`
/// variant representing the highest level node.
#[derive(Debug)]
pub enum SaphTree {
	Error(String),
	Stream(Vec<SaphTree>),
	Cmd(Vec<SaphTree>),
	CmdName(String),
	CmdPipe,
	List(Vec<SaphTree>),
	Null,
	Number(f32)
}

impl SaphTree {
	/// The public, top level parsing function intended to be used outside this
	/// module. It will accept a `String` object, and always return a `SaphTree`.
	/// ## Notes
	/// If a syntax error is found, it will return an `SaphTree::Error` variant.
	/// If an empty string is passed as input, the function returns `SaphTree::Null`.
	/// # Examples
	/// You can read the message from the error like this:
	/// ```rust
	/// use sapphire::parse::*;
	/// let code = String::from("5 6 3 -> out");
	/// let result = SaphTree::parse(&code);
	/// match result {
    ///     SaphTree::Error(e) => println!("{}", e),
    ///     _ => ()
	/// }
	/// ```
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
					'(' => {
						let lst_got = SaphTree::parse_list(context);
						match lst_got {
							SaphTree::Error(_) => return lst_got,
							_ => arr.push(lst_got)
						}
					},
					' ' | '\n' | '\t' => {
						context.next();
						()
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
					_ => return SaphTree::Error(format!("Expected number: '0' ... '9', found '{}'", context.next().unwrap()))
				},
				None => break
			}
		}
		match num.parse::<f32>() {
			Ok(pnf) => {
				if pos {
					return SaphTree::Number(pnf)
				} else {
					return SaphTree::Number(-pnf)
				}
			},
			Err(_) => return SaphTree::Error(format!("Found invalid number literal: '{}'", num))
		}
	}

	fn parse_list(context:&mut std::iter::Peekable<std::str::Chars>) -> Self {
		if *context.peek() != '(' {
			return SaphTree::Error(format!("Expected list beginner '(', got '{}'", context.next().unwrap()))
		}
		// consume the (
		context.next();
		let mut lst:Vec<SaphTree> = vec![];
		loop {
			match context.peek() {
				Some(ch) => match ch {
					' ' | '\n' | '\t'  => {
						context.next();
						()
					}
					'0' ... '9' => (),
					'-' => (),
					_ => return SaphTree::Error(format!("Expected list member, got '{}'", context.next().unwrap()))
				},
				// We shouldn't run out of chars before )
				None => return SaphTree::Error(format!("Expected end of lsit ')', got end of input"))
			}
		}
		SaphTree::List(lst)
	}
}

/// This module houses unit tests for Sapphire's parser.
#[cfg(test)]
mod parse_tests {
	use crate::parse::*;
	/// This test the number parsing rule of the `SaphTree` parser.
	/// Internally it will collect digit characters and try to parse via Rust's
	/// fish-head generic implementation. 
   #[test]
   fn parse_number_works() {
   	   let code = String::from("1004");
   	   let mut state = code.chars().peekable();
   	   match SaphTree::parse_number(&mut state, true) {
   	   	   SaphTree::Number(num) => assert_eq!(num, 1004 as f32),
   	   	   SaphTree::Error(e) => panic!("test failed got err {}", e),
   	   	   _ => panic!("Test parse number failed")
   	   }
   }

   #[test]
   fn parse_cmd_name_works() {
   	   let code = String::from("map");
   	   match SaphTree::parse_cmd_name(&mut code.chars().peekable()) {
   	   	    SaphTree::CmdName(name) => assert_eq!(name, String::from("map")),
   	   	    SaphTree::Error(e) => panic!("test failed got err {}", e),
   	   	    _ => panic!("Test parse cmd name failed")
   	   }
   }

   #[test]
   fn parse_cmd_works() {
   	   let code = String::from("map 5.332");
   	   match SaphTree::parse_cmd(&mut code.chars().peekable()) {
   	   	   SaphTree::Cmd(children) => {
   	   	   	   assert!(children.len() == 2);
   	   	   	   match &children[0] {
   	   	   	   	   SaphTree::CmdName(name) => assert_eq!(*name, String::from("map")),
   	   	   	   	   SaphTree::Error(e) => panic!("test failed got err {}", e),
   	   	   	   	   _ => panic!("Failed to parse cmd name inside cmd")
   	   	   	   }

   	   	   	   match &children[1] {
   	   	   	   	   SaphTree::Number(num) => assert_eq!(5.332, *num),
   	   	   	   	   SaphTree::Error(e) => panic!("test failed got err {}", e),
   	   	   	   	   _ => panic!("Failed to find a parsed number")
   	   	   	   }
   	   	   },
   	   	   SaphTree::Error(e) => panic!("test failed got err {}", e),
   	   	   _ => panic!("Test parse cmd failed")
   	   }
   }

   #[test]
   fn parse_stream_works() {
   	   let code = String::from("in 4 3 2  | map 2.3");
   	   match SaphTree::parse_stream(&mut code.chars().peekable()) {
   	   	     SaphTree::Stream(nodes) => {
   	   	     	  assert!(nodes.len() == 3);
   	   	     	  match &nodes[0] {
   	   	     	  	  SaphTree::Cmd(_) => (),
   	   	     	  	  SaphTree::Error(e) => panic!("test failed got err {}", e),
   	   	     	  	  _ => panic!("Failed to parse first command")
   	   	     	  }

   	   	     	  match &nodes[1] {
   	   	     	  	  SaphTree::CmdPipe => (),
   	   	     	  	  SaphTree::Error(e) => panic!("test failed got err {}", e),
   	   	     	  	  _ => panic!("Failed to parse command pipe") 	     	  	  
   	   	     	  }

   	   	     	  match &nodes[2] {
   	   	     	  	  SaphTree::Cmd(_) => (),
   	   	     	  	  SaphTree::Error(e) => panic!("test failed got err {}", e),
   	   	     	  	  _ => panic!("Failed to parse second command")
   	   	     	  }
   	   	     },
   	   	     SaphTree::Error(e) => panic!("test failed got err {}", e),
   	   	     _ => panic!("test for parse_stream failed")
   	   }
   }
}
