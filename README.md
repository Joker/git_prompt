# Simple git prompt for bash/zsh

Git repository information inside a shell prompt.

![git_prompt](https://repository-images.githubusercontent.com/266870139/b26c1300-2903-11eb-94cc-c2d2a5cc6592)

## Installation

```console
cargo install --git https://github.com/Joker/git_prompt
```
or
```console
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
