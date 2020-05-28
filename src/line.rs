const GREEN_L: &str = "\x1b[38;5;28m";
const BLUE_L: &str = "\x1b[1;94m";
const YELLOW: &str = "\x1b[0;33m";
// const RESET: &str = "\x1b[0m";

pub fn zsh_ps1(git: String) -> String {
	format!(
		// "{3}┌─{1}%n@%m {2}%~ {0}{3}\n└ >_ ",
		"{3}┌─{1}%n {2}%~ {0}{3}\n└ >_ ",
		git, GREEN_L, BLUE_L, YELLOW
	)
}

pub fn bash_ps1(git: String) -> String {
	format!(
		"{3}┌─{1}\\u@\\h {2}\\W {0}{3}\n└ >_ ",
		git, GREEN_L, BLUE_L, YELLOW
	)
}
