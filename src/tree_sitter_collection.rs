use tree_sitter_highlight::HighlightConfiguration;
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
    let mut highlights = tree_sitter_typescript::HIGHLIGHT_QUERY.to_owned();
    highlights.push_str(tree_sitter_javascript::HIGHLIGHT_QUERY);

    let mut locals = tree_sitter_typescript::LOCALS_QUERY.to_owned();
    locals.push_str(tree_sitter_javascript::LOCALS_QUERY);

    let conf = HighlightConfiguration::new(
      tree_sitter_typescript::language_typescript(),
      &highlights,
      tree_sitter_javascript::INJECTION_QUERY,
      &locals,
    )
    .unwrap();

    TreeSitterCollection { conf }
  }

  pub fn tsx() -> TreeSitterCollection {
    let mut highlights = tree_sitter_javascript::JSX_HIGHLIGHT_QUERY.to_owned();
    highlights.push_str(tree_sitter_typescript::HIGHLIGHT_QUERY);
    highlights.push_str(tree_sitter_javascript::HIGHLIGHT_QUERY);

    let mut locals = tree_sitter_typescript::LOCALS_QUERY.to_owned();
    locals.push_str(tree_sitter_javascript::LOCALS_QUERY);

    let conf = HighlightConfiguration::new(
      tree_sitter_typescript::language_tsx(),
      &highlights,
      tree_sitter_javascript::INJECTION_QUERY,
      &locals,
    )
    .unwrap();

    TreeSitterCollection { conf }
  }
  pub fn javascript() -> TreeSitterCollection {
    let conf = HighlightConfiguration::new(
      tree_sitter_javascript::language(),
      tree_sitter_javascript::HIGHLIGHT_QUERY,
      tree_sitter_javascript::INJECTION_QUERY,
      tree_sitter_javascript::LOCALS_QUERY,
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
      tree_sitter_toml::language(),
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
