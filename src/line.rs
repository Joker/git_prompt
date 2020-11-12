static GREEN_L: &str = "%{\x1b[38;5;28m%}";
static BLUE_L: &str = "%{\x1b[1;94m%}";
static YELLOW: &str = "%{\x1b[0;33m%}";
static RESET: &str = "%{\x1b[0m%}";

pub fn zsh_ps1(git: String) -> String {
	format!(
		// "{3}┌─{1}%n@%m {2}%~ {0}{3}\n└ >_{4} ",
		"{3}┌─{1}%n {2}%~ {0}{3}\n└ >_{4} ",
		git, GREEN_L, BLUE_L, YELLOW, RESET
	)
}

pub fn bash_ps1(git: String) -> String {
	format!(
		// "{3}┌─{1}\\u@\\h {2}\\W {0}{3}\n└ >_{4} ",
		"{3}┌─{1}\\u {2}\\W {0}{3}\n└ >_{4} ",
		git, GREEN_L, BLUE_L, YELLOW, RESET
	)
	.replace("%{", "")
	.replace("%}", "")
}
