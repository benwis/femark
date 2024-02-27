use crate::tree_sitter_collection::TreeSitterCollection;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;
use tree_sitter::QueryError;
use tree_sitter_highlight::HighlightConfiguration;

pub(crate) static LANGS: Lazy<Langs> = Lazy::new(|| Langs::new().unwrap());

pub struct Langs {
    pub langs: HashMap<&'static str, Arc<Lang>>,
}

pub struct Lang {
    pub conf: Option<HighlightConfiguration>,
    pub name: &'static str,
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
            let mut c = TreeSitterCollection::nix().conf;
            c.configure(&highlight_names);
            let c = Lang {
                conf: Some(c),
                name: "Nix code",
            };
            let c = Arc::new(c);
            res.langs.insert("nix", c);
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
                name: "Javascript React code",
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
                name: "Python code",
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
            res.langs.insert("dockerfile", c);
        }
        {
            let mut c = TreeSitterCollection::json().conf;
            c.configure(&highlight_names);
            let c = Lang {
                conf: Some(c),
                name: "JSON",
            };
            let c = Arc::new(c);
            res.langs.insert("json", c);
        }

        Ok(res)
    }

    pub fn get(&self, k: &str) -> Option<&Lang> {
        self.langs.get(k).map(|x| x.as_ref())
    }
}
