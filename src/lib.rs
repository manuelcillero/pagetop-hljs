//! <div align="center">
//!
//! <h1>HighlightJS</h1>
//!
//! <p><a href="https://docs.rs/pagetop">PageTop</a> package to display beautiful code snippets on web pages using the versatile <a href="https://highlightjs.org">highlight.js</a> JavaScript library.</p>
//!
//! [![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?style=for-the-badge)](https://github.com/manuelcillero/pagetop-hljs#-license)
//! [![API Docs](https://img.shields.io/docsrs/pagetop-hljs?label=API%20Docs&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-hljs)
//! [![Crates.io](https://img.shields.io/crates/v/pagetop-hljs.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-hljs)
//! [![Downloads](https://img.shields.io/crates/d/pagetop-hljs.svg?style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-hljs)
//!
//! <br>
//! </div>
//!
//! ## Usage
//!
//! Add `pagetop-hljs` to your `Cargo.toml`:
//!
//! ```rust
//! [dependencies]
//! pagetop-hljs = "<Version>"
//! ```
//!
//! Add `pagetop_hljs::HighlightJS` to your dependencies package:
//!
//! ```rust
//! use pagetop::prelude::*;
//!
//! impl PackageTrait for MyPackage {
//!     // ...
//!     fn dependencies(&self) -> Vec<PackageRef> {
//!         vec![
//!             // ...
//!             &pagetop_hljs::HighlightJS,
//!             // ...
//!         ]
//!     }
//!
//!     fn configure_service(&self, scfg: &mut service::web::ServiceConfig) {
//!         cfg.route("/", service::web::get().to(hljs_sample));
//!     }
//!     // ...
//! }
//! ```
//!
//! And put your code snippets on web pages:
//!
//! ```rust
//! use pagetop_hljs::prelude::*;
//!
//! #[service::get("/")]
//! async fn hljs_sample(request: HttpRequest) -> ResultPage<Markup, ErrorPage> {
//!     Page::new(request)
//!         .with_component(Snippet::with(
//!             HljsLang::Rust,
//!             r###"
//! // This is the main function.
//! fn main() {
//!     // Print text to the console.
//!     println!("Hello World!");
//! }
//!             "###,
//!         ))
//!         .render()
//! }
//! ```

#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/manuelcillero/pagetop/main/static/theme/favicon.ico"
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/manuelcillero/pagetop-hljs/main/static/pagetop_hljs.png"
)]

use pagetop::prelude::*;

pub mod config;

mod lang;
pub use lang::HljsLang;

mod theme;
pub use theme::HljsTheme;

mod mode;
pub use mode::HljsMode;

mod context;
pub use context::HljsContext;

mod snippet;
pub use snippet::Snippet;

/// The package Prelude.
pub mod prelude {
    pub use crate::{config, HljsContext, HljsLang, HljsMode, HljsTheme, Snippet};
}

static_locales!(LOCALES_HLJS);

static_files!(hljs);

// Highlight.js library version.
const HLJS_VERSION: &str = "11.7.0";

/// Implements [`PackageTrait`].
pub struct HighlightJS;

impl PackageTrait for HighlightJS {
    fn description(&self) -> L10n {
        L10n::t("hljs_description", &LOCALES_HLJS)
    }

    fn actions(&self) -> Vec<ActionBox> {
        actions![action::page::AfterPrepareBody::new(after_prepare_body)]
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        service_for_static_files!(cfg, hljs => "/hljs");
    }
}

// Sets up page assets depending on whether the 'core' or 'common' highlight.js library is used.
fn after_prepare_body(page: &mut Page) {
    let cx = page.context();

    if cx.is_hljs_enabled() {
        if let Some(languages) = cx.hljs_languages() {
            match cx.hljs_mode() {
                HljsMode::Core => {
                    cx.set_assets(AssetsOp::AddJavaScript(
                        JavaScript::at("/hljs/js/core.min.js")
                            .with_version(HLJS_VERSION)
                            .with_mode(ModeJS::Normal),
                    ));
                    for l in languages {
                        cx.set_assets(AssetsOp::AddJavaScript(
                            JavaScript::at(HljsLang::to_url(l))
                                .with_version(HLJS_VERSION)
                                .with_mode(ModeJS::Normal),
                        ));
                    }
                }
                _ => {
                    cx.set_assets(AssetsOp::AddJavaScript(
                        JavaScript::at("/hljs/js/highlight.min.js")
                            .with_version(HLJS_VERSION)
                            .with_mode(ModeJS::Normal),
                    ));
                }
            }

            // Configure highlight.js (disabling language autodetection).
            #[rustfmt::skip]
            cx.set_assets(AssetsOp::AddHeadScript(
                HeadScript::named("highlight.js").with_code(concat_string!("
                    hljs.configure({
                        tabReplace: '", " ".repeat(config::SETTINGS.hljs.tabsize), "',
                        languages: [],
                    });
                    hljs.highlightAll();
                ")),
            ));

            cx.set_assets(AssetsOp::AddStyleSheet(
                StyleSheet::at(HljsTheme::to_url(cx.hljs_theme().to_string()))
                    .with_version(HLJS_VERSION),
            ));
        }
    }
}
