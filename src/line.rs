use crate::git::{BLUE_L, CYAN, GREEN_L, RESET, YELLOW};

pub static ZSH: &str = r#"
ps1func () {
	PROMPT="$(git_prompt zsh ps1)"
}
precmd_functions+=(ps1func)
"#;
pub static BASH: &str = r#"
ps1func () {
	PS1="$(git_prompt bash ps1)"
}
PROMPT_COMMAND=ps1func
"#;

pub fn zsh_ps1(git: String) -> String {
	format!(
		"{0}┌─{1}%n {2}%~ {3}{0}\n└ >_{4} ", // %n@%m
		YELLOW, GREEN_L, BLUE_L, git, RESET
	)
}

pub fn bash_ps1(git: String) -> String {
	format!(
		"{0}┌─{1}\\u {2}\\w {3}{0}\n└ >_{4} ", // \\u@\\h
		CYAN, GREEN_L, BLUE_L, git, RESET
	)
	.replace("%{", "")
	.replace("%}", "")
}
