//! **HighlightJS** (`pagetop-hljs`) is a [PageTop](https://docs.rs/pagetop) module that displays
//! beautiful code snippets on web pages using the versatile [highlight.js](https://highlightjs.org)
//! JavaScript library.
//!
//! ## Usage
//!
//! Add the dependency `pagetop_hljs` to `Cargo.toml`:
//!
//! ```rust
//! [dependencies]
//! pagetop-hljs = "<Version>"
//! ```
//!
//! Add the dependency `pagetop_hljs::HighlightJS` to the module that uses it:
//!
//! ```rust
//! use pagetop::prelude::*;
//!
//! impl ModuleTrait for MyModule {
//!     // ...
//!     fn dependencies(&self) -> Vec<ModuleStaticRef> {
//!         vec![
//!             // ...
//!             &pagetop_hljs::HighlightJS
//!             // ...
//!         ]
//!     }
//!
//!     fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
//!         cfg.service(hljs_sample);
//!     }
//!     // ...
//! }
//! ```
//!
//! Now you can add code snippets on web pages:
//!
//! ```rust
//! use pagetop_hljs::prelude::*;
//!
//! #[service::get("/")]
//! async fn hljs_sample(request: service::HttpRequest) -> ResultPage<Markup, FatalError> {
//!     Page::new(request)
//!         .with_in(
//!             "content",
//!             Snippet::with(
//!                 HljsLang::Rust,
//!                 r###"
//! // This is the main function.
//! fn main() {
//!     // Print text to the console.
//!     println!("Hello World!");
//! }
//!                 "###,
//!             ),
//!         )
//!         .render()
//! }
//! ```
//!
//! ## Note
//!
//! HighlightJS uses [`ActionAfterPrepareBody`](pagetop::response::page::ActionAfterPrepareBody)
//! with a weight of 99 to add page assets. If you use this action to alter HighlightJS rendering,
//! such as specifying the theme for snippets, please ensure that your action has a weight lower
//! than 99. Default 0 is ok.

#![doc(html_favicon_url = "https://pagetop.cillero.es/theme/favicon.ico")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/manuelcillero/pagetop-hljs/main/static/pagetop_hljs.png")]

use pagetop::prelude::*;

use std::collections::HashSet;

pub mod component;
pub mod config;

mod lang;
pub use lang::HljsLang;

mod theme;
pub use theme::HljsTheme;

/// The HighlighJS Prelude.
pub mod prelude {
    pub use crate::component::{Snippet, COMPONENT_SNIPPET};
    pub use crate::HighlightJS;
    pub use crate::HljsLang;
    pub use crate::HljsTheme;
}

use_handle!(MODULE_HLJS);

use_locale!(LOCALE_HLJS);

use_static!(hljs);

// Library version.
const VERSION_HLJS: &str = "11.7.0";

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
        vec![action!(ActionAfterPrepareBody => after_prepare_body, 99)]
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

fn after_prepare_body(page: &mut Page) {
    let context = page.context();

    // The PARAM_HLJS_DISABLED parameter is set by disable_hljs(). If true, the library will be
    // disabled, preventing loading and syntax highlighting.
    if let Some(true) = context.get_param::<bool>(PARAM_HLJS_DISABLED) {
        return;
    }

    // The PARAM_HLJS_LANGS parameter stores languages (separated by semicolons) enabled by
    // enable_language(). If empty, the library will not be loaded.
    if let Some(languages) = context.get_param::<String>(PARAM_HLJS_LANGS) {
        // The PARAM_HLJS_LIB parameter is modified by force_core_lib() and force_common_lib(). It
        // takes values "core" or "common" based on the invoked function. If not assigned, the
        // config::LIB value is used, which defaults to config::SETTINGS.hljs.library or "core".
        match context
            .get_param::<String>(PARAM_HLJS_LIB)
            .unwrap_or(config::HLJS_LIB.to_owned())
            .as_str()
        {
            "core" => {
                context.alter(ContextOp::AddJavaScript(
                    JavaScript::at("/hljs/js/core.min.js")
                        .with_version(VERSION_HLJS)
                        .with_mode(ModeJS::Normal),
                ));
                let languages: HashSet<&str> = languages.split(';').collect();
                for l in languages {
                    context.alter(ContextOp::AddJavaScript(
                        JavaScript::at(HljsLang::to_url(l))
                            .with_version(VERSION_HLJS)
                            .with_mode(ModeJS::Normal),
                    ));
                }
            }
            _ => {
                context.alter(ContextOp::AddJavaScript(
                    JavaScript::at("/hljs/js/highlight.min.js")
                        .with_version(VERSION_HLJS)
                        .with_mode(ModeJS::Normal),
                ));
            }
        }

        // Configure highlight.js (disabling language autodetection).
        context.alter(ContextOp::AddHeadScript(
            HeadScript::named("highlight.js").with_code(
                concat_string!(
                    r###"
    hljs.configure({
        tabReplace: '"###,
                    " ".repeat(config::SETTINGS.hljs.tabsize),
                    r###"',
        languages: [],
    });
    hljs.highlightAll();
"###
                )
                .as_str(),
            ),
        ));

        // The PARAM_HLJS_THEME parameter stores the theme enabled by enable_theme(). If empty, the
        // config::THEME value is used, which defaults to config::SETTINGS.hljs.theme or
        // HljsTheme::Default.
        let theme = context
            .get_param::<String>(PARAM_HLJS_THEME)
            .unwrap_or(config::HLJS_THEME.to_string());
        context.alter(ContextOp::AddStyleSheet(
            StyleSheet::at(HljsTheme::to_url(theme.as_str())).with_version(VERSION_HLJS),
        ));
    }
}
