use eyre::Result;
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};
pub struct TreeSitterCollection {
  pub conf: HighlightConfiguration,
}

impl TreeSitterCollection {
  pub fn rust() -> TreeSitterCollection {
    let rust_conf = HighlightConfiguration::new(
      tree_sitter_rust::language(),
      tree_sitter_rust::HIGHLIGHT_QUERY,
      "",
      "",
    )
    .unwrap();

    TreeSitterCollection { conf: rust_conf }
  }
  pub fn typescript() -> TreeSitterCollection {
    let ts_conf = HighlightConfiguration::new(
      tree_sitter_typescript::language_typescript(),
      tree_sitter_typescript::HIGHLIGHT_QUERY,
      "",
      "",
    )
    .unwrap();

    TreeSitterCollection { conf: ts_conf }
  }

  pub fn tsx() -> TreeSitterCollection {
    let conf = HighlightConfiguration::new(
      tree_sitter_typescript::language_tsx(),
      tree_sitter_typescript::HIGHLIGHT_QUERY,
      "",
      "",
    )
    .unwrap();

    TreeSitterCollection { conf }
  }
  pub fn javascript() -> TreeSitterCollection {
    let conf = HighlightConfiguration::new(
      tree_sitter_javascript::language(),
      tree_sitter_javascript::HIGHLIGHT_QUERY,
      tree_sitter_javascript::INJECTION_QUERY,
      "",
    )
    .unwrap();

    TreeSitterCollection { conf }
  }
  pub fn jsx() -> TreeSitterCollection {
    let conf = HighlightConfiguration::new(
      tree_sitter_javascript::language(),
      tree_sitter_javascript::JSX_HIGHLIGHT_QUERY,
      tree_sitter_javascript::INJECTION_QUERY,
      "",
    )
    .unwrap();

    TreeSitterCollection { conf }
  }
  pub fn go() -> TreeSitterCollection {
    let conf = HighlightConfiguration::new(
      tree_sitter_go::language(),
      tree_sitter_go::HIGHLIGHT_QUERY,
      "",
      "",
    )
    .unwrap();

    TreeSitterCollection { conf }
  }
  pub fn c() -> TreeSitterCollection {
    let conf = HighlightConfiguration::new(
      tree_sitter_c::language(),
      tree_sitter_c::HIGHLIGHT_QUERY,
      "",
      "",
    )
    .unwrap();

    TreeSitterCollection { conf }
  }
  pub fn html() -> TreeSitterCollection {
    let conf = HighlightConfiguration::new(
      tree_sitter_html::language(),
      tree_sitter_html::HIGHLIGHT_QUERY,
      tree_sitter_html::INJECTION_QUERY,
      "",
    )
    .unwrap();

    TreeSitterCollection { conf }
  }
  pub fn toml() -> TreeSitterCollection {
    let conf = HighlightConfiguration::new(
      tree_sitter_html::language(),
      tree_sitter_toml::HIGHLIGHT_QUERY,
      "",
      "",
    )
    .unwrap();

    TreeSitterCollection { conf }
  }
  pub fn python() -> TreeSitterCollection {
    let conf = HighlightConfiguration::new(
      tree_sitter_python::language(),
      tree_sitter_python::HIGHLIGHT_QUERY,
      "",
      "",
    )
    .unwrap();

    TreeSitterCollection { conf }
  }
  pub fn dockerfile() -> TreeSitterCollection {
    let conf = HighlightConfiguration::new(tree_sitter_dockerfile::language(), "", "", "").unwrap();

    TreeSitterCollection { conf }
  }
  pub fn json() -> TreeSitterCollection {
    let conf = HighlightConfiguration::new(
      tree_sitter_json::language(),
      tree_sitter_json::HIGHLIGHT_QUERY,
      "",
      "",
    )
    .unwrap();

    TreeSitterCollection { conf }
  }
}
