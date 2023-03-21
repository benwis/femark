# femark

A blazingly fast  markdown to html parser and syntax highlighter built using the pulldown-cmark and tree-sitter-highlight crate natively. PRs are welcome. Very much a WIP

## Install

```sh
cargo add femark
```

## Use
The package exposes one function that will process your markdown and compile it to HTML. It will also generate a table of contents for you with your heading tags and their respective level. If you have no headings, toc should will be None, but if there is an error parsing your markdown, it will throw an error.

It is recomended to run this on the server, since it has a fairly large package size.

```rust
 use femark::{HTMLOutput, process_markdown_to_html};
 let HTMLOutput{content, toc} = process_markdown_to_html('# Hello, World!');
```

## Supported Languages

- Rust
- Typescript
- Tsx
- Javascript
- Jsx
- Dockerfile
- Python
- Nix
- Go
- C
- HTML
- TOML
- JSON

Currently the supported languages are driven mostly by my needs, but I am open to PRs to add additional language support if they are popular. 

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