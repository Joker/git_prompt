use crate::git::{BLUE_L, GREEN_L, RESET, YELLOW};

pub fn zsh_ps1(git: String) -> String {
	format!(
		"{0}┌─{1}%n {2}%~ {3}{0}\n└ >_{4} ", // %n@%m
		YELLOW, GREEN_L, BLUE_L, git, RESET
	)
}

pub fn bash_ps1(git: String) -> String {
	format!(
		"{0}┌─{1}\\u {2}\\W {3}{0}\n└ >_{4} ", // \\u@\\h
		YELLOW, GREEN_L, BLUE_L, git, RESET
	)
	.replace("%{", "")
	.replace("%}", "")
}
