//! **HighlightJS** is a [PageTop](https://docs.rs/pagetop) module that displays beautiful code
//! snippets on web pages using the versatile [highlight.js](https://highlightjs.org/) JavaScript
//! library.
//!
//! ## Usage:
//!
//! Add to `Cargo.toml` dependency to `pagetop-hljs`:
//!
//! ```rust
//! [dependencies]
//! pagetop-hljs = "<Version>"
//! ```
//!
//! Add to PageTop module (or application) dependency to **HighlightJS**:
//!
//! ```rust
//! use pagetop::prelude::*;
//!
//! impl ModuleTrait for ModuleName {
//!     // ...
//!     fn dependencies(&self) -> Vec<ModuleStaticRef> {
//!         vec![
//!             // ...
//!             &pagetop_hljs::HighlightJS
//!             // ...
//!         ]
//!     }
//!     // ...
//! }
//! ```
//!
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

// Library version.
const VERSION_HLJS: &str = "11.8.0";

// Context parameters.
const PARAM_HLJS_LIB: &str = "hljs.lib";
const PARAM_HLJS_LANGS: &str = "hljs.langs";
const PARAM_HLJS_THEME: &str = "hljs.theme";
const PARAM_HLJS_DISABLED: &str = "hljs.disabled";

/// Implements [`ModuleTrait`](pagetop::core::module::ModuleTrait) and specific module API.
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
    /// Enables a new language for processing code snippets. At least one language must be enabled
    /// to load the library.
    pub fn enable_language(&self, language: &HljsLang, cx: &mut Context) -> &Self {
        let languages = match cx.get_param::<String>(PARAM_HLJS_LANGS) {
            Some(previous) => concat_string!(previous, ";", language.to_string()),
            None => language.to_string(),
        };
        cx.set_param::<String>(PARAM_HLJS_LANGS, languages);
        self
    }

    /// Enables a new theme for displaying code snippets. The same theme is used for all snippets.
    pub fn enable_theme(&self, theme: HljsTheme, cx: &mut Context) -> &Self {
        cx.set_param::<String>(PARAM_HLJS_THEME, theme.to_string());
        self
    }

    /// Disables the library, preventing syntax highlighting from being applied to code snippets.
    pub fn disable_hljs(&self, cx: &mut Context) -> &Self {
        cx.set_param::<bool>(PARAM_HLJS_DISABLED, true);
        self
    }

    /// Enables the use of the core library, regardless of the `config::SETTINGS.hljs.library`
    /// configuration setting. This mode utilizes the core of the library and preloads only the
    /// languages enabled for snippets in the same context.
    pub fn force_core_lib(&self, cx: &mut Context) -> &Self {
        cx.set_param::<String>(PARAM_HLJS_LIB, "core".to_owned());
        self
    }

    /// Enables the use of the common library, regardless of the `config::SETTINGS.hljs.library`
    /// configuration setting. This mode uses a version of the library that includes almost 40
    /// languages in a single larger preload. If a code snippet requires a language that is not in
    /// the library, syntax highlighting will not be applied.
    pub fn force_common_lib(&self, cx: &mut Context) -> &Self {
        cx.set_param::<String>(PARAM_HLJS_LIB, "common".to_owned());
        self
    }
}

fn before_render_page(page: &mut Page) {
    // The PARAM_HLJS_DISABLED parameter is set by disable_hljs(). If true, the library will be
    // disabled, preventing loading and syntax highlighting.
    if let Some(true) = page.context().get_param::<bool>(PARAM_HLJS_DISABLED) {
        return;
    }

    // The PARAM_HLJS_LANGS parameter stores languages (separated by semicolons) enabled by
    // enable_language(). If empty, the library will not be loaded.
    if let Some(languages) = page.context().get_param::<String>(PARAM_HLJS_LANGS) {
        // The PARAM_HLJS_LIB parameter is modified by force_core_lib() and force_common_lib(). It
        // takes values "core" or "common" based on the invoked function. If not assigned, the
        // config::LIB value is used, which defaults to config::SETTINGS.hljs.library or "core".
        let library = page
            .context()
            .get_param::<String>(PARAM_HLJS_LIB)
            .unwrap_or(config::LIB.to_owned());
        page.context().alter(ContextOp::AddJavaScript(
            JavaScript::located(concat_string!("/hljs/js/", library, ".min.js"))
                .with_version(VERSION_HLJS)
                .with_mode(ModeJS::Normal),
        ));

        // Languages.
        let languages: HashSet<&str> = languages.split(';').collect();
        for l in languages {
            page.context().alter(ContextOp::AddJavaScript(
                JavaScript::located(HljsLang::to_url(l))
                    .with_version(VERSION_HLJS)
                    .with_mode(ModeJS::Normal),
            ));
        }

        // Configure (disabling language autodetection).
        page.context().alter(ContextOp::AddCodeScript(
            CodeScript::named("/hljs/code/highlight.js").with_code(concat_string!(
                r#"
                    hljs.configure({
                        tabReplace: "#,
                "'",
                " ".repeat(config::SETTINGS.hljs.tabsize),
                "',",
                r#"
                        languages: [],
                    });
                    hljs.highlightAll();
                "#
            )),
        ));

        // The PARAM_HLJS_THEME parameter stores the theme enabled by enable_theme(). If empty, the
        // config::THEME value is used, which defaults to config::SETTINGS.hljs.theme or
        // HljsTheme::Default.
        let theme = page
            .context()
            .get_param::<String>(PARAM_HLJS_THEME)
            .unwrap_or(config::THEME.to_string());
        page.context().alter(ContextOp::AddStyleSheet(
            StyleSheet::located(HljsTheme::to_url(theme.as_str())).with_version(VERSION_HLJS),
        ));
    }
}
