pub static RED: &str = "%{\x1b[0;31m%}";
pub static RED_L: &str = "%{\x1b[1;31m%}";
pub static GREEN: &str = "%{\x1b[0;32m%}";
pub static GREEN_L: &str = "%{\x1b[38;5;28m%}";
pub static BLUE_L: &str = "%{\x1b[1;94m%}";
pub static YELLOW: &str = "%{\x1b[0;33m%}";
pub static YELLOW_L: &str = "%{\x1b[1;33m%}";
pub static MAGENTA: &str = "%{\x1b[0;35m%}";
pub static MAGENTA_L: &str = "%{\x1b[1;35m%}";
pub static CYAN: &str = "%{\x1b[0;36m%}";
pub static RESET: &str = "%{\x1b[0m%}";

pub static PROMPT: &str = r#"
ps1func () {
	PROMPT="$(git_prompt ps1)"
}
precmd_functions+=(ps1func)
"#;

pub fn ps1(git: String) -> String {
	format!(
		"{0}┌─{1}%n {2}%~ {3}{0}\n└ >_{4} ", // %n@%m
		YELLOW, GREEN_L, BLUE_L, git, RESET
	)
}
