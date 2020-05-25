use std::env;
mod git;
mod line;

fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() > 1 && args[1] == "zsh" {
		line::ps1();
	} else {
		git::prompt();
	}
}
