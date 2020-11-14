# lazycomplete

Lazy loading for shell completion scripts.

## Status

WIP

## Usage

```sh
# bash
source <(lazycomplete \
  gh 'gh completion' \
  lab 'lab completion' \
  carapace 'carapace _carapace' \
)

# elvish
eval (lazycomplete ^
  gh 'gh completion' ^
  lab 'lab completion' ^
  carapace 'carapace _carapace' ^
|slurp)


# fish
lazycomplete \
  gh 'gh completion' \
  lab 'lab completion' \
  carapace 'carapace _carapace' \
| eval

# powershell
lazycomplete `
  gh 'gh completion' `
  lab 'lab completion' `
  carapace 'carapace _carapace' `
| Out-String | Invoke-Expression

# xonsh
exec($(lazycomplete \
  gh 'gh completion' \
  lab 'lab completion' \
  carapace 'carapace _carapace' \
))

# zsh
source <(lazycomplete \
  gh 'gh completion' \
  lab 'lab completion' \
  carapace 'carapace _carapace' \
)
```
