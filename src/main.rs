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
				"zsh" => print!("{}", git::prompt()),
				_ => print!("{}", git::prompt().replace("%{", "").replace("%}", "")),
			}
		}
		3 => {
			let cmd = &args[1];
			match &cmd[..] {
				"zsh" => print!("{}", line::zsh_ps1(git::prompt())),
				_ => print!("{}", line::bash_ps1(git::prompt())),
			}
		}
		_ => print!("{}", git::prompt().replace("%{", "").replace("%}", "")),
	}
}
