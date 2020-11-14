# lazycomplete

Lazy loading for shell completion scripts.

## Status

WIP

## Usage

```sh

# elvish
eval (lazycomplete ^
  gh 'gh completion' ^
  lab 'lab completion' ^
  carapace 'carapace _carapace' ^
|slurp)
```
