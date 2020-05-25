# Simple git prompt for bash/zsh

Git repository information inside a shell prompt.

## Installation

### zsh

`~/.zshrc`
```
setopt PROMPT_SUBST
PROMPT=$'%n@%m %1~ \$(git_prompt) $ '
```

### bash

`~/.bashrc`
```
PS1='\u@\h \W $(git_prompt) \$ '
```