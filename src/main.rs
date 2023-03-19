use git::prompt;
use std::env;

mod git;
mod line;

fn main() {
	// let arg = env::args().skip(1).next();
	let args: Vec<String> = env::args().collect();

	match args.len() {
		2 => {
			let cmd = &args[1];
			match &cmd[..] {
				"zsh" => print!("{}", prompt()),
				_ => print!("{}", prompt().replace("%{", "").replace("%}", "")),
			}
		}
		3 => {
			let cmd = &args[1];
			match &cmd[..] {
				"zsh" => print!("{}", line::zsh_ps1(prompt())),
				_ => print!("{}", line::bash_ps1(prompt())),
			}
		}
		_ => print!("{}", prompt().replace("%{", "").replace("%}", "")),
	}
}
