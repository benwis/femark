# femark

A markdown to html parser and syntax highlighter built using Rust's pulldown-cmark and tree-sitter-highlight crate natively for Node's Foreign Function Interface. PRs are welcome. Very much a WIP

## Install

```sh
npm i @benwis/femark
```

## Use
```ts
processMarkdownToHtml('# Hello, World!');
```

## Supported Languages

- Rust
- Typescript
- Tsx
- Javascript
- Jsx
- Dockerfile
- Python
- Go
- C
- HTML
- TOML
- JSON

Currently the supported languages are driven mostly by my needs, but I am open to PRs to add additional language support if they are popular. I am currently investigating a way to add language support at runtime, but currently there is no way to do so.

## Theme

By default, this package does not style your code blocks, merely decorates the elements with classes that range from `hh0` to `hh20`. The indices refer to the elements in this list:
```rust
let highlight_names = [
      "attribute",
      "constant",
      "function.builtin",
      "function",
      "keyword",
      "operator",
      "property",
      "punctuation",
      "punctuation.bracket",
      "punctuation.delimiter",
      "string",
      "string.special",
      "tag",
      "type",
      "type.builtin",
      "variable",
      "variable.builtin",
      "variable.parameter",
      "comment",
      "macro",
      "label",
    ]

```
For example, hh0 would refer to an attribute and hh20 would be a label. You'll want to add some css classes for each attribute. Because this is a common tree-sitter theme, if you search for neovim themes that support tree-sitter, you can find items like `TSFunction` and `TSAttribute` with examples. A basic theme is provided below:
```css
.hh4 {
  color: purple;
}

.hh3 {
  color: blue;
}

.hh13 {
  color: pink;
}

.hh10 {
  color: green;
}

.hh5 {
  color: gray;
}

.hh18 {
  color: lightgray;
}
```

## M1 support
Currently Github Actions does not seem to fully support compiling C++ dependencies for the M1. I am unsure if it is even possible to cross compile from a x64 VM, and it does not offer arm VMs. Since it is unlikely you will be hosting the server on an M1 machine, you can still develop on the M1 as I do by running the `npm install` and `npm run build` commands from the root of the package.