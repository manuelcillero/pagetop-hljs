//! **HighlightJS** (`pagetop-hljs`) is a [PageTop](https://docs.rs/pagetop) package that displays
//! beautiful code snippets on web pages using the versatile [highlight.js](https://highlightjs.org)
//! JavaScript library.
//!
//! ## Usage
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```rust
//! [dependencies]
//! pagetop-hljs = "<Version>"
//! ```
//!
//! Add the dependency `pagetop_hljs::HighlightJS` to your package:
//!
//! ```rust
//! use pagetop::prelude::*;
//!
//! impl Package for MyPackage {
//!     // ...
//!     fn dependencies(&self) -> Vec<PackageRef> {
//!         vec![
//!             // ...
//!             &pagetop_hljs::HighlightJS
//!             // ...
//!         ]
//!     }
//!
//!     fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
//!         scfg.route("/", service::web::get().to(hljs_sample));
//!     }
//!     // ...
//! }
//! ```
//!
//! Now you can put code snippets on web pages:
//!
//! ```rust
//! use pagetop_hljs::prelude::*;
//!
//! #[service::get("/")]
//! async fn hljs_sample(request: service::HttpRequest) -> ResultPage<Markup, ErrorPage> {
//!     Page::new(request)
//!         .with_component_in(
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
//! HighlightJS hooks [`ActionAfterPrepareBody`](pagetop::response::page::ActionAfterPrepareBody)
//! using a weight of 99 to add page assets. If you hook this action to alter HighlightJS rendering,
//! such as setting the theme for snippets (using [`set_theme()`](crate::HighlightJS::set_theme())),
//! ensure that your action has a weight lower than 99. Default 0 is ok.

#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/manuelcillero/pagetop/main/static/theme/favicon.ico"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/manuelcillero/pagetop-hljs/main/static/pagetop_hljs.png"
)]

use pagetop::prelude::*;

use std::collections::HashSet;

pub mod config;

pub mod component;

mod lang;
pub use lang::HljsLang;

mod theme;
pub use theme::HljsTheme;

/// The HighlighJS Prelude.
pub mod prelude {
    pub use crate::component::Snippet;
    pub use crate::HighlightJS;
    pub use crate::HljsLang;
    pub use crate::HljsTheme;
}

static_locales!(LOCALES_HLJS);

static_files!(hljs);

// Library version.
const VERSION_HLJS: &str = "11.7.0";

// Context parameters.
const PARAM_HLJS_LIB: &str = "hljs.lib";
const PARAM_HLJS_LANGS: &str = "hljs.langs";
const PARAM_HLJS_THEME: &str = "hljs.theme";
const PARAM_HLJS_DISABLED: &str = "hljs.disabled";

/// Implements [`Package`](pagetop::core::package::Package) and specific package API.
pub struct HighlightJS;

impl PackageTrait for HighlightJS {
    fn name(&self) -> L10n {
        L10n::t("package_name", &LOCALES_HLJS)
    }

    fn description(&self) -> L10n {
        L10n::t("package_description", &LOCALES_HLJS)
    }

    fn actions(&self) -> Vec<Action> {
        actions![action::page::AfterPrepareBody::new(after_prepare_body).with_weight(99)]
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        service_for_static_files!(cfg, hljs => "/hljs");
    }
}

impl HighlightJS {
    /// Add a new language for processing code snippets. It is necessary to add at least one
    /// language to load the *highlight.js* library. The [`Snippet`](component::Snippet) component
    /// automatically adds the required language.
    pub fn add_language(&self, language: &HljsLang, cx: &mut Context) -> &Self {
        let languages = match cx.get_param::<String>(PARAM_HLJS_LANGS) {
            Some(previous) => concat_string!(previous, ";", language.to_string()),
            None => language.to_string(),
        };
        cx.set_param::<String>(PARAM_HLJS_LANGS, languages);
        self
    }

    /// Change the theme for displaying code snippets. The same theme is used for all snippets in
    /// the given context.
    pub fn set_theme(&self, theme: HljsTheme, cx: &mut Context) -> &Self {
        cx.set_param::<String>(PARAM_HLJS_THEME, theme.to_string());
        self
    }

    /// Disable the loading of the *highlight.js* library, preventing syntax highlighting in code
    /// snippets.
    pub fn disable_hljs(&self, cx: &mut Context) -> &Self {
        cx.set_param::<bool>(PARAM_HLJS_DISABLED, true);
        self
    }

    /// Force the use of the *highlight.js* ***core*** library, ignoring the
    /// `config::SETTINGS.hljs.library` configuration setting. This mode utilizes the core of the
    /// library and preloads only the languages enabled for snippets in the same context.
    pub fn force_core_lib(&self, cx: &mut Context) -> &Self {
        cx.set_param::<String>(PARAM_HLJS_LIB, "core".to_owned());
        self
    }

    /// Force the use of the *highlight.js* ***common*** library, ignoring the
    /// `config::SETTINGS.hljs.library` configuration setting. This mode uses a version of the
    /// library that includes almost 40 languages in a single preload. If a code snippet requires a
    /// language that is not in the library, syntax highlighting will not be applied.
    pub fn force_common_lib(&self, cx: &mut Context) -> &Self {
        cx.set_param::<String>(PARAM_HLJS_LIB, "common".to_owned());
        self
    }
}

fn after_prepare_body(page: &mut Page) {
    let cx = page.context();

    // PARAM_HLJS_DISABLED parameter is set by disable_hljs(). If true, the library will be
    // disabled, preventing loading and syntax highlighting.
    if let Some(true) = cx.get_param::<bool>(PARAM_HLJS_DISABLED) {
        return;
    }

    // PARAM_HLJS_LANGS parameter stores languages (separated by semicolons) enabled by
    // add_language(). If empty, the library will not be loaded.
    if let Some(languages) = cx.get_param::<String>(PARAM_HLJS_LANGS) {
        // PARAM_HLJS_LIB parameter is modified by force_core_lib() and force_common_lib(). It
        // takes values "core" or "common" based on the invoked function. If not assigned, the
        // config::HLJS_LIB value is used, which defaults to config::SETTINGS.hljs.library or
        // "core".
        match cx
            .get_param::<String>(PARAM_HLJS_LIB)
            .unwrap_or(config::HLJS_LIB.to_owned())
            .as_str()
        {
            "core" => {
                cx.alter(ContextOp::AddJavaScript(
                    JavaScript::at("/hljs/js/core.min.js")
                        .with_version(VERSION_HLJS)
                        .with_mode(ModeJS::Normal),
                ));
                let languages: HashSet<&str> = languages.split(';').collect();
                for l in languages {
                    cx.alter(ContextOp::AddJavaScript(
                        JavaScript::at(HljsLang::to_url(l))
                            .with_version(VERSION_HLJS)
                            .with_mode(ModeJS::Normal),
                    ));
                }
            }
            _ => {
                cx.alter(ContextOp::AddJavaScript(
                    JavaScript::at("/hljs/js/highlight.min.js")
                        .with_version(VERSION_HLJS)
                        .with_mode(ModeJS::Normal),
                ));
            }
        }

        // Configure highlight.js (disabling language autodetection).
        cx.alter(ContextOp::AddHeadScript(
            HeadScript::named("highlight.js").with_code(concat_string!(
                r###"
    hljs.configure({
        tabReplace: '"###,
                " ".repeat(config::SETTINGS.hljs.tabsize),
                r###"',
        languages: [],
    });
    hljs.highlightAll();
                "###
            )),
        ));

        // PARAM_HLJS_THEME parameter stores the theme enabled by set_theme(). If empty, the
        // config::HLJS_THEME value is used, which defaults to config::SETTINGS.hljs.theme or
        // HljsTheme::Default.
        let theme = cx
            .get_param::<String>(PARAM_HLJS_THEME)
            .unwrap_or(config::HLJS_THEME.to_string());
        cx.alter(ContextOp::AddStyleSheet(
            StyleSheet::at(HljsTheme::to_url(theme.as_str())).with_version(VERSION_HLJS),
        ));
    }
}
