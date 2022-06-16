use crate::code_format::highlight_string_snippet;
use once_cell::sync::Lazy;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

static THEME_SET: Lazy<ThemeSet> = Lazy::new(|| ThemeSet::load_defaults());
pub struct SyntectFormatter<'a> {
    token: Option<&'a str>,
    extension: Option<&'a str>,
}

impl<'a> SyntectFormatter<'a> {
    pub fn new() -> Self {
        SyntectFormatter {
            token: None,
            extension: None,
        }
    }
    pub fn token(mut self, token: &'a str) -> Self {
        self.token = Some(token);
        self
    }
    // pub fn extension(mut self, extension: &'a str) -> Self {
    //     self.extension = Some(extension);
    //     self
    // }
    // Note: this is NOT ESCAPED!!
    // Do ammonia to sanitize this first!!!!
    pub fn highlight_snippet(&self, snippet: &str) -> String {
        // thread_local!(static SYN_SET :SyntaxSet = SyntaxSet::load_defaults_newlines());
        static SYN_SET: Lazy<SyntaxSet> = Lazy::new(|| SyntaxSet::load_defaults_newlines());

        let theme = &THEME_SET.themes["base16-ocean.dark"];

        // SYN_SET.with(|s| {
        let mut syntax = self.token.and_then(|tok| SYN_SET.find_syntax_by_token(tok));
        syntax = syntax.or_else(|| {
            self.extension
                .and_then(|ext| SYN_SET.find_syntax_by_extension(ext))
        });
        if let Some(syntax) = syntax {
            // TODO find a way to avoid inline css in the syntect formatter
            highlight_string_snippet(snippet, syntax, theme, &SYN_SET)
        } else {
            format!("<pre><code>{}</code></pre>", snippet)
        }
        // })
    }
}
