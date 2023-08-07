mod git;
mod line;

fn main() {
	let args: Vec<String> = std::env::args().collect();

	match args.len() {
		3 => match (args[1].as_str(), args[2].as_str()) {
			("zsh", "init") => print!("{}", line::ZSH),
			("zsh", "ps1") => print!("{}", line::zsh_ps1(git::prompt())),
			("bash", "init") => print!("{}", line::BASH),
			_ => print!(
				"{}",
				line::bash_ps1(git::prompt().replace("%{", "").replace("%}", ""))
			),
		},
		_ => print!("{}", git::prompt().replace("%{", "").replace("%}", "")),
	}
}
