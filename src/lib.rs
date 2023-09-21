mod tree_sitter_collection;
mod parser;
mod langs;

use eyre::Result;
use pulldown_cmark::Parser;
use tree_sitter_highlight::{Highlight, HighlightEvent, Highlighter};
use pulldown_cmark_frontmatter::FrontmatterExtractor;
use crate::parser::{Toc, options, generate_toc, process_stream, process_stream_with_frontmatter, OwnedFrontmatter};
use crate::langs::{LANGS, Lang};

#[derive(Default)]
#[non_exhaustive]
pub struct HTMLOutput {
    pub toc: Option<String>,
    pub content: String,
    pub frontmatter: Option<OwnedFrontmatter>,
}
/// Processes markdown to HTML, and highlights the code blocks. If you have frontmatter
/// in your markdown files, it will be rendered as well.
pub fn process_markdown_to_html(input: &str) -> Result<HTMLOutput> {
    process_markdown_to_html_with_frontmatter(input, false)
}
/// Processes markdown to html and syntax highlights the code blocks.
/// Scans for frontmatter, and parses it. A H1 in the frontmatter will be returned.
/// while the frontmatter code block will not.
pub fn process_markdown_to_html_with_frontmatter(input: &str, extract_frontmatter: bool) -> Result<HTMLOutput> {
    let parser = Parser::new_ext(&input, options());
    let parser2 = Parser::new_ext(&input, options());
    let langs = &LANGS;
    let mut toc: Toc = Vec::new();
    let mut output: Vec<u8> = Vec::new();
    let mut frontmatter: Option<OwnedFrontmatter> = None;
    //   let stream = WideImages::new(parser);
    let mut frontmatter_parser = FrontmatterExtractor::new(parser);

    match extract_frontmatter{
        true => process_stream_with_frontmatter(&mut frontmatter_parser, langs, &mut toc, &mut output, &mut frontmatter),
        false => process_stream(parser2, langs, &mut toc, &mut output),
    };

    let toc_html = generate_toc(&toc);

    match String::from_utf8(output) {
        Ok(s) => Ok(HTMLOutput {
            toc: toc_html,
            content: s,
            frontmatter,
        }),
        Err(e) => Err(HighlightError::StringGenerationError(e.to_string()).into()),
    }
}

#[derive(Debug, thiserror::Error)]
enum HighlightError {
    #[error("language not recognized")]
    NoLang,
    #[error("no highlighter for language")]
    NoHighlighter,
    #[error("could not build highlighter: {0}")]
    CouldNotBuildHighlighter(String),
    #[error("Could not generate utf8 String: {0}")]
    StringGenerationError(String),
}

impl HighlightError {
    fn benign(&self) -> bool {
        matches!(self, Self::NoLang | Self::NoHighlighter)
    }
}

fn highlight_code(
    w: &mut dyn std::fmt::Write,
    source: &str,
    lang: &Option<&Lang>,
) -> std::result::Result<(), HighlightError> {
    let lang = lang.ok_or(HighlightError::NoLang)?;
    let conf = lang.conf.as_ref().ok_or(HighlightError::NoHighlighter)?;

    let mut highlighter = Highlighter::new();
    let highlights = highlighter
        .highlight(conf, source.as_bytes(), None, |_| None)
        .map_err(|e| HighlightError::CouldNotBuildHighlighter(format!("{:?}", e)))?;
    for highlight in highlights {
        let highlight = highlight.unwrap();
        match highlight {
            HighlightEvent::Source { start, end } => {
                write_code_escaped(w, &source[start..end]).unwrap();
            }
            HighlightEvent::HighlightStart(Highlight(i)) => {
                write!(w, r#"<i class=hh{}>"#, i).unwrap();
            }
            HighlightEvent::HighlightEnd => {
                write!(w, r#"</i>"#).unwrap();
            }
        }
    }

    Ok(())
}

fn write_code_escaped(w: &mut dyn std::fmt::Write, input: &str) -> Result<()> {
    let mut start: Option<usize> = None;

    for (i, c) in input.char_indices() {
        match c {
            '<' | '>' | '&' => {
                if let Some(start) = start.take() {
                    write!(w, "{}", &input[start..i])?;
                }
                match c {
                    '<' => write!(w, "&lt;")?,
                    '>' => write!(w, "&gt;")?,
                    '&' => write!(w, "&amp;")?,
                    _ => {}
                };
            }
            _ => {
                if start.is_none() {
                    start = Some(i)
                }
            }
        }
    }
    if let Some(start) = start.take() {
        write!(w, "{}", &input[start..])?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use super::*;
    use crate::HTMLOutput;

    #[test]
    fn test_write_code_escaped() {
        let mut out = String::new();
        write_code_escaped(&mut out, "The Vec<u8> type").unwrap();
        assert_eq!(&out, "The Vec&lt;u8&gt; type");

        out.clear();
        write_code_escaped(&mut out, "ParseResult<&str> Or Result<Vec<_>> && false").unwrap();
        assert_eq!(
            &out,
            "ParseResult&lt;&amp;str&gt; Or Result&lt;Vec&lt;_&gt;&gt; &amp;&amp; false"
        );
    }
    #[test]
    fn test_basic_highlighting() {
        let input = r#"```html
        <main class="potato">Hello World</main>
        ```"#;
        let HTMLOutput { content, .. } = process_markdown_to_html(input).unwrap();
        assert_eq!(content,"<div class=\"code-block\"><div class=\"language-tag\">HTML</div><pre class=\"code-block-inner\" data-lang=\"html\">        <i class=hh8>&lt;</i><i class=hh12>main</i> <i class=hh0>class</i>=\"<i class=hh10>potato</i>\"<i class=hh8>&gt;</i>Hello World<i class=hh8>&lt;/</i><i class=hh12>main</i><i class=hh8>&gt;</i>\n        ```</pre></div>");
    }

    #[test]
    fn test_nix_highlighting() {
        let input = r#"```nix
        rec {
            number_key = 5;
            list_key = [ number_key true "Hello" ];
        }
        ```"#;
        let HTMLOutput { content, .. } = process_markdown_to_html(input).unwrap();
        assert_eq!(content,"<div class=\"code-block\"><div class=\"language-tag\">Nix code</div><pre class=\"code-block-inner\" data-lang=\"nix\">        <i class=hh4>rec</i> <i class=hh8>{</i>\n            <i class=hh6><i class=hh6>number_key</i></i> <i class=hh9>=</i> 5<i class=hh9>;</i>\n            <i class=hh6><i class=hh6>list_key</i></i> <i class=hh9>=</i> <i class=hh8>[</i> <i class=hh15>number_key</i> <i class=hh16>true</i> <i class=hh10>\"Hello\"</i> <i class=hh8>]</i><i class=hh9>;</i>\n        <i class=hh8>}</i>\n        ```</pre></div>");
    }

    #[test]
    fn test_ts_highlighting() {
        let input = r#"```ts
       const user = {
  firstName: "Angela",
  lastName: "Davis",
  role: "Professor",
}

console.log(user.name)
        ```"#;
        let HTMLOutput { content, .. } = process_markdown_to_html(input).unwrap();
        assert_eq!(content,"<div class=\"code-block\"><div class=\"language-tag\">TypeScript code</div><pre class=\"code-block-inner\" data-lang=\"ts\">       <i class=hh4>const</i> <i class=hh15>user</i> <i class=hh5>=</i> <i class=hh8>{</i>\n  <i class=hh6>firstName</i>: <i class=hh10>\"Angela\"</i><i class=hh9>,</i>\n  <i class=hh6>lastName</i>: <i class=hh10>\"Davis\"</i><i class=hh9>,</i>\n  <i class=hh6>role</i>: <i class=hh10>\"Professor\"</i><i class=hh9>,</i>\n<i class=hh8>}</i>\n\n<i class=hh16>console</i><i class=hh9>.</i><i class=hh3>log</i><i class=hh8>(</i><i class=hh15>user</i><i class=hh9>.</i><i class=hh6>name</i><i class=hh8>)</i>\n        <i class=hh10>``</i>`</pre></div>");
    }
    #[test]
    fn test_frontmatter() {
        #[derive(Serialize, Deserialize)]
        pub struct ExampleAttributes {
            pub author: String,
            pub last_updated_at: String,
        }
        let input = include_str!("../samples/frontmatter_example.md");

        let HTMLOutput { content, frontmatter, .. } = process_markdown_to_html_with_frontmatter(input, true).unwrap();

       let Some(frontmatter) = frontmatter else {
        panic!("No frontmatter detected!");
       };
        assert_eq!(
            frontmatter.title.expect("title not detected"),
            "Frontmatter Example Document"
        );
        let code_block = frontmatter.code_block.unwrap();
        assert_eq!(code_block.language.as_deref(), Some("toml"));
        let attrs: ExampleAttributes = toml::from_str(&code_block.source).expect("invalid toml");
        assert_eq!(attrs.author, "https://fosstodon.org/@ecton");

        assert_eq!(content,"\n                        <h1>\n                            <a id=\"frontmatter-example-document\" class=\"anchor\" href=\"#frontmatter-example-document\">\n                                Frontmatter Example Document\n                            </a>\n                        </h1>\n                        \n<p>This is an example document with embedded frontmatter. The\npulldown-cmark-frontmatter crate removes the frontmatter code block so that it\ndoes not show up in the final output.</p>\n"
);
    }

 }
