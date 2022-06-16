#![deny(clippy::all)]
mod code_format;
mod escape;
mod syntect;
mod utils;

#[macro_use]
extern crate napi_derive;

use crate::syntect::SyntectFormatter;
// use ammonia::Builder;
// use once_cell::sync::Lazy;
use pulldown_cmark::{html, CodeBlockKind, Event, Parser, Tag};

struct EventIter<'a> {
  p: Parser<'a, 'a>,
}

impl<'a> EventIter<'a> {
  pub fn new(p: Parser<'a, 'a>) -> Self {
    EventIter { p }
  }
}
// static AMMONIA_BUILDER: Lazy<Builder<'static>> = Lazy::new(|| construct_ammonia_builder());
impl<'a> Iterator for EventIter<'a> {
  type Item = Event<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    let next = if let Some(v) = self.p.next() {
      v
    } else {
      return None;
    };
    if let &Event::Start(Tag::CodeBlock(_)) = &next {
      // Codeblock time!
      let mut text_buf = String::new();
      let mut next = self.p.next();
      loop {
        if let Some(Event::Text(ref s)) = next {
          text_buf += s;
        } else {
          break;
        }
        next = self.p.next();
      }
      let mut fmt = SyntectFormatter::new();
      match &next {
        Some(Event::End(Tag::CodeBlock(cb))) => {
          if let CodeBlockKind::Fenced(ref token) = cb {
            fmt = fmt.token(token);
          }
        }
        _ => panic!("Unexpected element inside codeblock mode {:?}", next),
      }
      let formatted = fmt.highlight_snippet(&text_buf);
      return Some(Event::Html(formatted.into()));
    }
    Some(next)
  }
}

// fn construct_ammonia_builder() -> Builder<'static> {
//     use std::iter;
//     let mut r = Builder::default();
//     // TODO: filter out everything that can have scr attributes.
//     // TODO: maybe replace all img's with their alt text?
//     r.rm_tags(iter::once("img"));
//     // TODO: do filtering of inline CSS
//     // (or even better: output classes instead of inline css)
//     r.add_tag_attributes("span", iter::once("style"));
//     r
// }

/// Renders a given markdown string to sanitized HTML
/// with formatted code blocks.
#[napi]
pub fn render_markdown(markdown: String) -> String {
  let p = Parser::new(&markdown);
  let ev_it = EventIter::new(p);
  let mut unsafe_html = String::new();
  html::push_html(&mut unsafe_html, ev_it);
  // AMMONIA_BUILDER.clean(&unsafe_html).to_string()
  unsafe_html
}
