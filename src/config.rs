//! Configuration settings for **HighlightJS** module.
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

use super::theme::THEMES;
use super::HljsTheme;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
/// Configuration settings for the [`[hljs]`](Hljs) section (see [`SETTINGS`]).
pub struct Settings {
    pub hljs: Hljs,
}
#[derive(Debug, Deserialize)]
/// [`Settings`] section `[hljs]`.
pub struct Hljs {
    /// You can utilize the ***core*** library to import the language specific to each code snippet
    /// enabled in the given context (see [`enable_language()`](crate::HighlightJS::enable_language())).
    /// Furthermore, the ***common*** library provides support for approximately 40 popular
    /// languages. However, if you exclusively rely on the common library, you will only have access
    /// to these preloaded languages.
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

use_config!(SETTINGS as Settings,
    // [hljs]
    "hljs.library" => "core",
    "hljs.theme"   => "default",
    "hljs.tabsize" => 4,
);

// Defaults to SETTINGS.hljs.library or "core".
pub(crate) static LIB: LazyStatic<&str> =
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

// Defaults to SETTINGS.hljs.theme or HljsTheme::Default.
pub(crate) static THEME: LazyStatic<&HljsTheme> = LazyStatic::new(|| {
    if let Some((theme, _)) = THEMES
        .iter()
        .find(|(_, &value)| value == SETTINGS.hljs.theme)
    {
        &theme
    } else {
        trace::warn!(
            "Unrecognized theme '{}' for HighlightJS, 'default' is assumed",
            SETTINGS.hljs.theme,
        );
        &HljsTheme::Default
    }
});
