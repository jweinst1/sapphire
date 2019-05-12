use std::env;

static HELP_MESSAGE: &'static str = "~~~ Sapphire Help ~~~";

fn main() {
	println!("{}", HELP_MESSAGE);
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
