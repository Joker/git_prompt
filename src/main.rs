#[cfg(feature = "bash")]
mod bash;
#[cfg(feature = "bash")]
use bash::{ps1, PROMPT};

#[cfg(not(feature = "bash"))]
mod zsh;
#[cfg(not(feature = "bash"))]
use zsh::{ps1, PROMPT};

mod git;

fn main() {
	let args: Vec<String> = std::env::args().collect();

	match args.len() {
		2 => match args[1].as_str() {
			"init" => print!("{}", PROMPT),
			"ps1" => print!("{}", ps1(git::prompt())),
			_ => print!("{}", git::prompt()),
		},
		_ => print!("{}", git::prompt().replace("%{", "").replace("%}", "")),
	}
}
