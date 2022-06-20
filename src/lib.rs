mod tree_sitter_collection;

#[macro_use]
extern crate napi_derive;

use crate::tree_sitter_collection::TreeSitterCollection;
use eyre::Result;
use once_cell::sync::Lazy;
use pulldown_cmark::{html, CodeBlockKind, Event, HeadingLevel, Options, Parser, Tag};
use serde::{Deserialize, Serialize};
use slug::slugify;
use std::{collections::HashMap, io::Cursor, sync::Arc};
use tracing::{debug, warn};
use tree_sitter::QueryError;
use tree_sitter_highlight::{Highlight, HighlightConfiguration, HighlightEvent, Highlighter};

fn options() -> Options {
  let mut options = Options::empty();
  options.insert(Options::ENABLE_TABLES);
  options.insert(Options::ENABLE_FOOTNOTES);
  options.insert(Options::ENABLE_STRIKETHROUGH);
  options.insert(Options::ENABLE_TASKLISTS);
  options
}

static LANGS: Lazy<Langs> = Lazy::new(|| Langs::new().unwrap());

pub struct Langs {
  langs: HashMap<&'static str, Arc<Lang>>,
}

pub struct Lang {
  conf: Option<HighlightConfiguration>,
  name: &'static str,
}

impl Langs {
  pub fn new() -> std::result::Result<Self, QueryError> {
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
    .iter()
    .cloned()
    .map(String::from)
    .collect::<Vec<_>>();

    let mut res = Self {
      langs: Default::default(),
    };

    {
      let mut c = TreeSitterCollection::go().conf;
      c.configure(&highlight_names);
      let c = Lang {
        conf: Some(c),
        name: "Go code",
      };
      let c = Arc::new(c);
      res.langs.insert("go", c);
    }
    {
      let mut c = TreeSitterCollection::c().conf;
      c.configure(&highlight_names);
      let c = Lang {
        conf: Some(c),
        name: "C code",
      };
      let c = Arc::new(c);
      res.langs.insert("c", c);
    }
    {
      let mut c = TreeSitterCollection::rust().conf;
      c.configure(&highlight_names);
      let c = Lang {
        conf: Some(c),
        name: "Rust code",
      };
      let c = Arc::new(c);
      res.langs.insert("rust", c);
    }
    {
      let mut c = TreeSitterCollection::javascript().conf;
      c.configure(&highlight_names);

      let c = Lang {
        conf: Some(c),
        name: "JavaScript code",
      };
      let c = Arc::new(c);
      res.langs.insert("javascript", Arc::clone(&c));
      res.langs.insert("js", c);
    }
    {
      let mut c = TreeSitterCollection::jsx().conf;
      c.configure(&highlight_names);
      let c = Lang {
        conf: Some(c),
        name: "jsx code",
      };
      let c = Arc::new(c);
      res.langs.insert("jsx", c);
    }
    {
      let mut c = TreeSitterCollection::typescript().conf;
      c.configure(&highlight_names);
      let c = Lang {
        conf: Some(c),
        name: "TypeScript code",
      };
      let c = Arc::new(c);
      res.langs.insert("typescript", Arc::clone(&c));
      res.langs.insert("ts", c);
    }
    {
      let mut c = TreeSitterCollection::tsx().conf;
      c.configure(&highlight_names);
      let c = Lang {
        conf: Some(c),
        name: "TypeScript React code",
      };
      let c = Arc::new(c);
      res.langs.insert("tsx", c);
    }
    {
      let mut c = TreeSitterCollection::toml().conf;
      c.configure(&highlight_names);
      let c = Lang {
        conf: Some(c),
        name: "TOML markup",
      };
      let c = Arc::new(c);
      res.langs.insert("toml", c);
    }
    {
      let mut c = TreeSitterCollection::html().conf;
      c.configure(&highlight_names);
      let c = Lang {
        conf: Some(c),
        name: "HTML",
      };
      let c = Arc::new(c);
      res.langs.insert("html", c);
    }
    {
      let mut c = TreeSitterCollection::html().conf;
      c.configure(&highlight_names);
      let c = Lang {
        conf: Some(c),
        name: "XML",
      };
      let c = Arc::new(c);
      res.langs.insert("xml", c);
    }
    {
      res.langs.insert(
        "shell",
        Arc::new(Lang {
          conf: None,
          name: "Shell session",
        }),
      );
    }
    {
      res.langs.insert(
        "pwsh",
        Arc::new(Lang {
          conf: None,
          name: "PowerShell session",
        }),
      );
    }
    {
      res.langs.insert(
        "pwsh-script",
        Arc::new(Lang {
          conf: None,
          name: "PowerShell script",
        }),
      );
    }
    {
      res.langs.insert(
        "raw",
        Arc::new(Lang {
          conf: None,
          name: "",
        }),
      );
    }
    {
      let mut c = TreeSitterCollection::python().conf;
      c.configure(&highlight_names);
      let c = Lang {
        conf: Some(c),
        name: "Python",
      };
      let c = Arc::new(c);
      res.langs.insert("python", c);
    }
    //   {
    //     let mut c = TreeSitterCollection::yaml().conf;
    //     c.configure(&highlight_names);
    //     let c = Lang {
    //       conf: Some(c),
    //       name: "YAML",
    //     };
    //     let c = Arc::new(c);
    //     res.langs.insert("yml", c);
    //   }
    {
      let mut c = TreeSitterCollection::dockerfile().conf;
      c.configure(&highlight_names);
      let c = Lang {
        conf: Some(c),
        name: "Dockerfile",
      };
      let c = Arc::new(c);
      res.langs.insert("Dockerfile", c);
    }
    {
      let mut c = TreeSitterCollection::json().conf;
      c.configure(&highlight_names);
      let c = Lang {
        conf: Some(c),
        name: "JSON",
      };
      let c = Arc::new(c);
      res.langs.insert("JSON", c);
    }

    Ok(res)
  }

  pub fn get(&self, k: &str) -> Option<&Lang> {
    self.langs.get(k).map(|x| x.as_ref())
  }
}

pub type Toc = Vec<TocEntry>;

#[derive(Serialize, Deserialize)]
pub struct TocEntry {
  // 1 through 6
  level: u8,
  // something like "The basics"
  text: String,
  // something like "the-basics"
  slug: String,
}

/// Processes markdown to html and syntax highlights the code blocks
#[napi]
pub fn process_markdown_to_html(input: String) -> String {
  let parser = Parser::new_ext(&input, options());
  let stream = parser;
  let langs = &LANGS;
  let mut toc: Toc = Vec::new();
  let mut output: Vec<u8> = Vec::new();
  //   let stream = WideImages::new(parser);

  struct Code {
    lang: String,
    source: String,
  }
  let mut current_code: Option<Code> = None;

  struct Heading {
    level: HeadingLevel,
    #[allow(dead_code)]
    frag: Option<String>,
    #[allow(dead_code)]
    class: Vec<String>,
    markup: String,
    plain_text: String,
  }
  let mut current_heading: Option<Heading> = None;

  let mut in_blockquote = false;
  let mut in_figcaption = false;

  let stream = stream.map(|ev| {
    debug!(?ev, "Got markdown event");
    match &ev {
      Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
        current_code = Some(Code {
          lang: lang.to_string(),
          source: Default::default(),
        });
        return Event::Text("".into());
      }
      Event::Start(Tag::Heading(level, frag, class)) if !in_blockquote && !in_figcaption => {
        current_heading = Some(Heading {
          level: *level,
          frag: frag.map(ToOwned::to_owned),
          class: class.iter().map(ToString::to_string).collect(),
          markup: "".into(),
          plain_text: "".into(),
        });
        return Event::Text("".into());
      }
      Event::Start(Tag::BlockQuote) => {
        in_blockquote = true;
      }
      Event::Code(contents) => {
        if let Some(current_heading) = &mut current_heading {
          current_heading
            .markup
            .push_str(&format!("<code>{contents}</code>"));
          current_heading.plain_text.push_str(contents.as_ref());
          return Event::Text("".into());
        }
      }
      Event::Html(html) => {
        if html.contains("<figcaption>") {
          in_figcaption = true;
        }
        if html.contains("</figcaption>") {
          in_figcaption = false;
        }
      }
      Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(_))) => {
        if let Some(current) = current_code.take() {
          let mut out: String = String::new();
          use std::fmt::Write;

          let lang = langs.get(&current.lang);
          write!(&mut out, r#"<div class="code-block">"#,).ok();

          let tag = lang.map(|l| l.name).unwrap_or(&current.lang);
          if !tag.is_empty() {
            write!(
              &mut out,
              r#"<div class="language-tag">{}</div>"#,
              lang.map(|l| l.name).unwrap_or(&current.lang)
            )
            .ok();
          }
          write!(
            &mut out,
            r#"<pre class="code-block-inner" data-lang={:?}>"#,
            current.lang
          )
          .ok();

          if let Err(e) = highlight_code(&mut out, &current.source, &lang) {
            if !e.benign() {
              warn!("Highlight error: {}", e);
            }
            write_code_escaped(&mut out, &current.source).ok();
          }
          write!(&mut out, "</pre></div>").ok();

          return Event::Html(out.into());
        }
      }
      Event::End(Tag::Heading(_, _, _)) => {
        if let Some(heading) = current_heading.take() {
          let tag = match heading.level {
            HeadingLevel::H1 => "h1",
            HeadingLevel::H2 => "h2",
            HeadingLevel::H3 => "h3",
            HeadingLevel::H4 => "h4",
            HeadingLevel::H5 => "h5",
            HeadingLevel::H6 => "h6",
          };
          let markup = &heading.markup;
          let anchor = slugify(&heading.plain_text);
          let href = format!("#{anchor}");

          let toc_entry = TocEntry {
            level: heading.level as u8,
            text: heading.markup.clone(),
            slug: anchor.clone(),
          };
          toc.push(toc_entry);

          return Event::Html(
            format!(
              r#"
                        <{tag}>
                            <a id="{anchor}" class="anchor" href="{href}">
                                {markup}
                            </a>
                        </{tag}>
                        "#
            )
            .into(),
          );
        }
      }
      Event::End(Tag::BlockQuote) => {
        in_blockquote = false;
      }
      Event::Text(text) => {
        if let Some(current) = current_code.as_mut() {
          current.source.push_str(text);
          return Event::Text("".into());
        } else if let Some(current) = current_heading.as_mut() {
          current.markup.push_str(text);
          current.plain_text.push_str(text);
          return Event::Text("".into());
        }
      }
      _ => {}
    }

    ev
  });

  html::write_html(Cursor::new(&mut output), stream).unwrap();

  match String::from_utf8(output) {
    Ok(html) => html,
    Err(_) => "".to_string(),
  }
  //   Ok(output)
}

#[derive(Debug, thiserror::Error)]
enum HighlightError {
  #[error("language not recognized")]
  NoLang,
  #[error("no highlighter for language")]
  NoHighlighter,
  #[error("could not build highlighter: {0}")]
  CouldNotBuildHighlighter(String),
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
  use super::*;

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
}
