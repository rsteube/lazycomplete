# lazycomplete

Lazy loading for shell completion scripts.

A lot of programs provide their own shell completion script by invoking them with a specific argument like `mybinary completion`. The easiest way to use it is to add the call to the shell init script (like `.bashrc`) which keeps the completion up to date. Add a couple of these and shell startup time is affected considerably though as a single invocation and subsequent parsing can take ~50-100ms. This tool generates a lazy loading script for given binaries where the actual completion script is resolved only when needed.
## Status

WIP

## Usage

Invoke with pairs of binary name and command to be invoked to create the completion:

```sh
# bash
source <(lazycomplete \
  gh 'gh completion' \
  lab 'lab completion' \
)

# elvish
eval (lazycomplete ^
  gh 'gh completion' ^
  lab 'lab completion' ^
|slurp)


# fish
lazycomplete \
  gh 'gh completion' \
  lab 'lab completion' \
| eval

# oil
source <(lazycomplete \
  gh 'gh completion' \
  lab 'lab completion' \
)

# powershell
lazycomplete `
  gh 'gh completion' `
  lab 'lab completion' `
| Out-String | Invoke-Expression

# xonsh
exec($(lazycomplete \
  gh 'gh completion' \
  lab 'lab completion' \
))

# zsh
source <(lazycomplete \
  gh 'gh completion' \
  lab 'lab completion' \
)
```
