use crate::bindings::{OwnedCodeBlock, OwnedFrontmatter};
use crate::langs::Langs;
use crate::{highlight_code, write_code_escaped};
use once_cell::sync::Lazy;
use pulldown_cmark::{html, CodeBlockKind, Event, HeadingLevel, Options, Tag};
use pulldown_cmark_frontmatter::{CodeBlock, Frontmatter, FrontmatterExtractor};
use serde::{Deserialize, Serialize};
use slug::slugify;
use std::io::Cursor;
use tracing::{debug, warn};
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

/// Takes a vector of TocEntrues and returns it as HTML
pub(crate) fn generate_toc(toc: &Toc) -> Option<String> {
    let mut toc_html = String::new();

    if !toc.is_empty() {
        toc_html.push_str("<ul class=\"table-of-contents\">");
        for entry in toc {
            toc_html.push_str(&format!(
                "<li class=\"toc-entry level-{}\"><a href=\"#{}\">{}</a></li>",
                entry.level, entry.slug, entry.text
            ));
        }
        toc_html.push_str("</ul>");
        return Some(toc_html);
    }
    None
}
struct Code {
    lang: String,
    source: String,
}

struct Heading {
    level: HeadingLevel,
    #[allow(dead_code)]
    frag: Option<String>,
    #[allow(dead_code)]
    class: Vec<String>,
    markup: String,
    plain_text: String,
}

impl From<CodeBlock<'_>> for OwnedCodeBlock {
    fn from(cb: CodeBlock) -> Self {
        Self {
            language: cb.language.map(|c| c.to_string()),
            source: cb.source.to_string(),
        }
    }
}

impl From<Frontmatter<'_>> for OwnedFrontmatter {
    fn from(fm: Frontmatter) -> Self {
        Self {
            title: fm.title.map_or(None, |t| Some(t)),
            code_block: fm.code_block.map(|t| t.into()),
        }
    }
}
pub fn process_stream<'a, T>(
    parser: T,
    langs: &Lazy<Langs, fn() -> Langs>,
    toc: &mut Toc,
    output: &mut Vec<u8>,
) where
    T: Iterator<Item = Event<'a>>,
{
    let mut current_code: Option<Code> = None;
    let mut current_heading: Option<Heading> = None;
    let mut in_blockquote = false;
    let mut in_figcaption = false;

    let stream = parser.map(|ev| {
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
    html::write_html(Cursor::new(output), stream).unwrap();
}

pub fn process_stream_with_frontmatter<'a, T>(
    parser: &mut FrontmatterExtractor<'a, T>,
    langs: &Lazy<Langs, fn() -> Langs>,
    toc: &mut Toc,
    output: &mut Vec<u8>,
    frontmatter_out: &mut Option<OwnedFrontmatter>,
) where
    T: Iterator<Item = Event<'a>>,
{
    let mut current_code: Option<Code> = None;
    let mut current_heading: Option<Heading> = None;
    let mut in_blockquote = false;
    let mut in_figcaption = false;

    parser.extract_buffered();
    *frontmatter_out = parser.frontmatter.clone().map(|f| f.into());
    let stream = parser.map(|ev| {
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
    html::write_html(Cursor::new(output), stream).unwrap();
}
pub(crate) fn options() -> Options {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options
}

