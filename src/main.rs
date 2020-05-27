use std::env;
mod git;
mod line;

fn main() {
	let arg = env::args().skip(1).next();

	match arg {
		Some(sh) => {
			if sh == "zsh" {
				print!("{}", line::zsh_ps1(git::prompt()))
			}
			if sh == "bash" {
				print!("{}", line::bash_ps1(git::prompt()))
			}
		}
		None => print!("{}", git::prompt()),
	}
}
