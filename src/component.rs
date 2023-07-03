//! Add a new component to incorporate code snippets on web pages.

use pagetop::prelude::*;

use super::{HighlightJS, HljsLang};

use_handle!(COMPONENT_HLJS);

#[rustfmt::skip]
#[derive(Default)]
/// Implements
/// [`ComponentTrait`](https://docs.rs/pagetop/latest/pagetop/core/component/trait.ComponentTrait.html)
/// and the specific API builder for the component.
pub struct Hljs {
    weight    : isize,
    renderable: Renderable,
    language  : HljsLang,
    code      : String,
}

impl ComponentTrait for Hljs {
    fn new() -> Self {
        Hljs::default()
    }

    fn handle(&self) -> Handle {
        COMPONENT_HLJS
    }

    fn weight(&self) -> isize {
        self.weight
    }

    fn is_renderable(&self, context: &Context) -> bool {
        (self.renderable.check)(context)
    }

    fn before_prepare_component(&mut self, cx: &mut Context) {
        HighlightJS.enable(self.language(), cx);
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

    fn as_ref_any(&self) -> &dyn AnyComponent {
        self
    }

    fn as_mut_any(&mut self) -> &mut dyn AnyComponent {
        self
    }
}

impl Hljs {
    // Hljs BUILDER.

    #[fn_builder]
    pub fn alter_weight(&mut self, weight: isize) -> &mut Self {
        self.weight = weight;
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
    pub fn alter_code(&mut self, code: String) -> &mut Self {
        self.code = code;
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
