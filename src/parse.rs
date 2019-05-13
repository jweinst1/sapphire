

#[derive(Debug)]
enum SaphPrsCmd {
	Error(String),
	Base,
	GotI,
	GotIn,
	CmdIn
}

impl SaphPrsCmd {
	fn digest(&mut self, ch:char) {
		match self {
			SaphPrsCmd::Base => match ch {
				'i' => *self = SaphPrsCmd::GotI,
				_ => *self = SaphPrsCmd::Error(format!("Got unexpected character '{}'", ch))
			},
			SaphPrsCmd::GotI => match ch {
				'n' => *self = SaphPrsCmd::GotIn,
				_ => *self = SaphPrsCmd::Error(format!("Expected 'n', got unexpected character '{}'", ch))
			},
			SaphPrsCmd::GotIn => match ch {
				' ' | '\n' | '\t' => *self = SaphPrsCmd::CmdIn,
				_ => *self = SaphPrsCmd::Error(format!("Expected ' ', '\\n', or '\\t', got unexpected character '{}'", ch))
			},
			_ => ()
		}
	}
}


#[derive(Debug)]
enum SaphPrsArg {
	Error(String),
	Base,
	GotN,
	GotNu,
	GotNul,
	Null,
	GotIntCh(String),
	Int(i32)
}

impl SaphPrsArg {
	fn digest(&mut self, ch:char) {
		match self {
			SaphPrsArg::Base => match ch {
				'0' ... '9' => *self = SaphPrsArg::GotIntCh(format!("{}", ch)),
				'N' => *self = SaphPrsArg::GotN,
				_ => *self = SaphPrsArg::Error(format!("Got unexpected character '{}' while looking for args", ch))
			},
			SaphPrsArg::GotN => (),
			SaphPrsArg::GotNu => (),
			SaphPrsArg::GotNul => (),
			SaphPrsArg::GotIntCh(itoken) => (),
			_ => ()
		}
	}
}
