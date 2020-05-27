const GREEN_L: &str = "\x1b[1;92m";
const BLUE_L: &str = "\x1b[1;94m";
const YELLOW: &str = "\x1b[0;33m";
const RESET: &str = "\x1b[0m";

pub fn zsh_ps1(git: String) -> String {
	format!(
		"{3}┌─{1}%u@%m {2}%~ {0}\n{3}└ >_ {4}",
		git, GREEN_L, BLUE_L, YELLOW, RESET
	)
}

pub fn bash_ps1(git: String) -> String {
	format!(
		"{3}┌─{1}\\u@\\h {2}\\W {0}\n{3}└ >_ {4}",
		git, GREEN_L, BLUE_L, YELLOW, RESET
	)
}
