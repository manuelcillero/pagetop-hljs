//! Configuration settings for HighlightJS module.
//!
//! Example:
//!
//! ```toml
//! [hljs]
//! library = "common"
//! theme = "zenburn"
//! tabsize = 8
//! ```
//!
//! Usage:
//!
//! ```rust
//! use pagetop_hljs::config;
//!
//! assert_eq!(config::SETTINGS.hljs.theme, "zenburn");
//! ```
//! See [`pagetop::config`](pagetop::config) to learn how **PageTop** read configuration files and
//! use settings.

use pagetop::prelude::*;

use super::theme::HLJS_THEMES;
use super::HljsTheme;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Type for HighlightJS configuration settings, section [`[hljs]`](Hljs) (used by [`SETTINGS`]).
pub struct Settings {
    pub hljs: Hljs,
}
#[derive(Debug, Deserialize)]
/// Struct for section `[hljs]` of [`Settings`] type.
pub struct Hljs {
    /// You can use the ***core*** library to import only the languages added by each snippet (see
    /// [`add_language()`](crate::HighlightJS::add_language())), or use the ***common*** library
    /// which supports around 40 popular languages. However, using the common library will restrict
    /// its usage to only these preloaded languages.
    /// Default value: *"core"*
    pub library: String,
    /// Default theme to display code snippets on web pages, written in *kebab-case* (see
    /// [`HljsTheme`](crate::HljsTheme)).
    /// Default value: *"default"*
    pub theme: String,
    /// Number of spaces for *tab* character.
    /// Default value: *4*
    pub tabsize: usize,
}

default_settings!(
    // [hljs]
    "hljs.library" => "core",
    "hljs.theme"   => "default",
    "hljs.tabsize" => 4,
);

// Defaults to valid SETTINGS.hljs.library or "core".
pub(crate) static HLJS_LIB: LazyStatic<&str> =
    LazyStatic::new(|| match SETTINGS.hljs.library.to_lowercase().as_str() {
        "core" => "core",
        "common" => "common",
        _ => {
            trace::warn!(
                "Unrecognized '{}' HighlightJS library, 'core' is assumed",
                SETTINGS.hljs.library,
            );
            "core"
        }
    });

// Defaults to valid SETTINGS.hljs.theme or HljsTheme::Default.
pub(crate) static HLJS_THEME: LazyStatic<&HljsTheme> = LazyStatic::new(|| {
    let theme = SETTINGS.hljs.theme.to_lowercase();
    if let Some((t, _)) = HLJS_THEMES.iter().find(|(_, &v)| v == theme) {
        t
    } else {
        trace::warn!(
            "Unrecognized theme '{}' for HighlightJS, 'default' is assumed",
            SETTINGS.hljs.theme,
        );
        &HljsTheme::Default
    }
});
