# Simple git prompt for bash/zsh

Git repository information inside a shell prompt.

![git_prompt](https://repository-images.githubusercontent.com/266870139/78024e00-1251-11eb-9ee9-7809becffeb6)

## Installation

```
make install
```

### zsh

`~/.zshrc`
```
setopt PROMPT_SUBST
PROMPT=$'%n@%m %1~ \$(git_prompt zsh) $ '
```

### bash

`~/.bashrc`
```
PS1='\u@\h \W $(git_prompt) \$ '
```
