//! Add a new component to put code snippets on web pages.

use pagetop::prelude::*;

use super::{HighlightJS, HljsLang};

#[rustfmt::skip]
#[derive(AutoDefault)]
/// Component to put code snippets on web pages.
pub struct Snippet {
    weight    : Weight,
    renderable: Renderable,
    language  : HljsLang,
    snippet   : String,
}

impl ComponentTrait for Snippet {
    fn new() -> Self {
        Snippet::default()
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn is_renderable(&self, cx: &Context) -> bool {
        (self.renderable.check)(cx)
    }

    fn setup_before_prepare(&mut self, cx: &mut Context) {
        HighlightJS.add_language(self.language(), cx);
    }

    fn prepare_component(&self, _cx: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            pre {
                code class=(concat_string!("language-", self.language().to_string())) {
                    (self.snippet())
                }
            }
        })
    }
}

impl Snippet {
    pub fn with(language: HljsLang, code: impl Into<String>) -> Self {
        Snippet::new().with_language(language).with_snippet(code)
    }

    // Hljs BUILDER.

    #[fn_builder]
    pub fn alter_weight(&mut self, value: Weight) -> &mut Self {
        self.weight = value;
        self
    }

    #[fn_builder]
    pub fn alter_renderable(&mut self, check: FnIsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    #[fn_builder]
    pub fn alter_language(&mut self, language: HljsLang) -> &mut Self {
        self.language = language;
        self
    }

    #[fn_builder]
    pub fn alter_snippet(&mut self, snippet: impl Into<String>) -> &mut Self {
        self.snippet = snippet.into().trim().to_owned();
        self
    }

    // Hljs GETTERS.

    pub fn language(&self) -> &HljsLang {
        &self.language
    }

    pub fn snippet(&self) -> &String {
        &self.snippet
    }
}
