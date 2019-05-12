
/// The `SapphireToken` is the result of parsing a stream of
/// Sapphire source code. Each token represents a syntax component
/// housed between pipes `|`.
#[derive(Debug)]
#[allow(dead_code)]
enum SapphireToken {
	IntRange(String),
	CmdPrint,
	EmptyList
}
