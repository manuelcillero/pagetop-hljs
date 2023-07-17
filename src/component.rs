//! Add a new component to put code snippets on web pages.

use pagetop::prelude::*;

use super::{HighlightJS, HljsLang};

create_handle!(COMPONENT_SNIPPET);

#[rustfmt::skip]
#[derive(Default)]
/// Component to put code snippets on web pages.
pub struct Snippet {
    weight    : Weight,
    renderable: Renderable,
    language  : HljsLang,
    code      : String,
}

impl ComponentTrait for Snippet {
    fn new() -> Self {
        Snippet::default()
    }

    fn handle(&self) -> Handle {
        COMPONENT_SNIPPET
    }

    fn weight(&self) -> Weight {
        self.weight
    }

    fn is_renderable(&self, context: &Context) -> bool {
        (self.renderable.check)(context)
    }

    fn before_prepare_component(&mut self, cx: &mut Context) {
        HighlightJS.add_language(self.language(), cx);
    }

    fn prepare_component(&self, _context: &mut Context) -> PrepareMarkup {
        PrepareMarkup::With(html! {
            pre {
                code class=(concat_string!("language-", self.language().to_string())) {
                    (self.code())
                }
            }
        })
    }
}

impl Snippet {
    pub fn with(language: HljsLang, code: impl Into<String>) -> Self {
        Snippet::new().with_language(language).with_code(code)
    }

    // Hljs BUILDER.

    #[fn_builder]
    pub fn alter_weight(&mut self, value: Weight) -> &mut Self {
        self.weight = value;
        self
    }

    #[fn_builder]
    pub fn alter_renderable(&mut self, check: IsRenderable) -> &mut Self {
        self.renderable.check = check;
        self
    }

    #[fn_builder]
    pub fn alter_language(&mut self, language: HljsLang) -> &mut Self {
        self.language = language;
        self
    }

    #[fn_builder]
    pub fn alter_code(&mut self, code: impl Into<String>) -> &mut Self {
        self.code = code.into().trim().to_owned();
        self
    }

    // Hljs GETTERS.

    pub fn language(&self) -> &HljsLang {
        &self.language
    }

    pub fn code(&self) -> &String {
        &self.code
    }
}
