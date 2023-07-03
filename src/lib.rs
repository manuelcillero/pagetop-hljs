//! The [PageTop](https://docs.rs/pagetop) module **HighlightJS** displays beautiful code snippets
//! on web pages using the [highlight.js](https://highlightjs.org/) library:
//!
//! * Supports **90+** coding languages.
//! * Choose from all **95+** available themes.
//! * Provides a component for adding code snippets.
//! * Highlight multi-line blocks of code.
//! * Detects `language-` and `lang-` class prefixes.
//! * Customize the *highlight.js* init JavaScript.
//! * Smart loading of CSS & JS assets.
//!
//! Example:
//! ```rust
//! use pagetop_hljs::HighlightJS;
//!
//! // Enable Highlight.js in context using the highlight.js default theme:
//! HighLightJS.enable(cx).with_theme("default", cx);
//! ```

use pagetop::prelude::*;

use std::collections::HashSet;

pub mod component;
pub mod config;

mod lang;
pub use lang::HljsLang;

mod theme;
pub use theme::HljsTheme;

use_handle!(MODULE_HLJS);

use_locale!(LOCALE_HLJS);

use_static!(hljs);

const VERSION_HLJS: &str = "11.8.0";

const PARAM_HLJS_LANGS: &str = "hljs.langs";
const PARAM_HLJS_THEME: &str = "hljs.theme";

/// Implements
/// [`ModuleTrait`](https://docs.rs/pagetop/latest/pagetop/core/module/trait.ModuleTrait.html)
/// and the specific API for the module.
pub struct HighlightJS;

impl ModuleTrait for HighlightJS {
    fn handle(&self) -> Handle {
        MODULE_HLJS
    }

    fn name(&self) -> L10n {
        L10n::t("module_name", &LOCALE_HLJS)
    }

    fn description(&self) -> L10n {
        L10n::t("module_description", &LOCALE_HLJS)
    }

    fn actions(&self) -> Vec<Action> {
        vec![action!(actions::page::ActionBeforeRenderPage => before_render_page)]
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        serve_static_files!(cfg, "/hljs", hljs);
    }
}

impl HighlightJS {
    /// Enable in context the **Highlight.js** library.
    pub fn enable(&self, language: &HljsLang, cx: &mut Context) -> &Self {
        let languages = match cx.get_param::<String>(PARAM_HLJS_LANGS) {
            Some(l) => concat_string!(l, ";", language.to_string()),
            None => language.to_string(),
        };
        cx.set_param::<String>(PARAM_HLJS_LANGS, languages);
        self
    }

    /// Enable in context the **Highlight.js** theme.
    pub fn with_theme(&self, theme: HljsTheme, cx: &mut Context) -> &Self {
        cx.set_param::<String>(PARAM_HLJS_THEME, theme.to_string());
        self
    }

    /// Disable in context the **Highlight.js** library.
    pub fn disable(&self, cx: &mut Context) -> &Self {
        cx.remove_param(PARAM_HLJS_LANGS);
        self
    }
}

fn before_render_page(page: &mut Page) {
    if let Some(l) = page.context().get_param::<String>(PARAM_HLJS_LANGS) {
        // Theme.
        let theme = page
            .context()
            .get_param::<String>(PARAM_HLJS_THEME)
            .unwrap_or(theme::THEME.to_string());
        page.context().alter(ContextOp::AddStyleSheet(
            StyleSheet::located(concat_string!("/hljs/css/", theme, ".min.css"))
                .with_version(VERSION_HLJS),
        ));

        // Highlight.js core.
        page.context().alter(ContextOp::AddJavaScript(
            JavaScript::located("/hljs/js/core.min.js")
                .with_version(VERSION_HLJS)
                .with_mode(ModeJS::Normal),
        ));

        // Languages.
        let languages: HashSet<&str> = l.split(';').collect();
        for lang in languages {
            page.context().alter(ContextOp::AddJavaScript(
                JavaScript::located(concat_string!("/hljs/js/lang/", lang, ".min.js"))
                    .with_version(VERSION_HLJS)
                    .with_mode(ModeJS::Normal),
            ));
        }

        // Configure and enable.
        page.context().alter(ContextOp::AddCodeScript(
            CodeScript::located("/hljs/code/highlight.js").with_code(
                r#"
                    hljs.configure({
                        tabReplace: '    ', // 4 spaces
                        languages: [],      // Languages used for auto-detection
                    });
                    hljs.highlightAll();
                "#
                .to_owned(),
            ),
        ));
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
*/
