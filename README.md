<div align="center">

  <h1>PageTop HighlightJS</h1>

  [![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg?style=for-the-badge)](#license)
  [![API Docs](https://img.shields.io/docsrs/pagetop-hljs?label=API%20Docs&style=for-the-badge&logo=Docs.rs)](https://docs.rs/pagetop-hljs)
  [![Crates.io](https://img.shields.io/crates/v/pagetop-hljs.svg?style=for-the-badge&logo=ipfs)](https://crates.io/crates/pagetop-hljs)
  [![Downloads](https://img.shields.io/crates/d/pagetop-hljs.svg?style=for-the-badge&logo=transmission)](https://crates.io/crates/pagetop-hljs)

</div>

---

**HighlightJS** (`pagetop-hljs`) is a PageTop package that displays beautiful code snippets on web
pages using the versatile [highlight.js](https://highlightjs.org/) JavaScript library.

## Features

  * Supports **90+** coding languages.
  * Choose from all **95+** available themes.
  * Provides a component for adding code snippets.
  * Highlight multi-line blocks of code.
  * Detects `language-` and `lang-` class prefixes.
  * Customize the *highlight.js* init JavaScript.
  * Smart loading of CSS & JS assets.

## Usage

Add the following to your `Cargo.toml`:

```rust
[dependencies]
pagetop-hljs = "<Version>"
```

Add the dependency `pagetop_hljs::HighlightJS` to your module:

```rust
use pagetop::prelude::*;

impl PackageTrait for MyPackage {
    // ...
    fn dependencies(&self) -> Vec<PackageRef> {
        vec![
            // ...
            &pagetop_hljs::HighlightJS
            // ...
        ]
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        cfg.route("/", service::web::get().to(hljs_sample));
    }
    // ...
}
```

Now you can put code snippets on web pages:

```rust
use pagetop_hljs::prelude::*;

#[service::get("/")]
async fn hljs_sample(request: service::HttpRequest) -> ResultPage<Markup, ErrorPage> {
    Page::new(request)
        .with_component_in(
            "content",
            Snippet::with(
                HljsLang::Rust,
                r###"
// This is the main function.
fn main() {
    // Print text to the console.
    println!("Hello World!");
}
                "###,
            ),
        )
        .render()
}
```


# 📦 About PageTop

[PageTop](https://docs.rs/pagetop) is an opinionated [Rust](https://www.rust-lang.org) web
development framework to build secure and modular Server-Side Rendering (SSR) web solutions.


# 🚧 Warning

**PageTop** framework is currently in active development. The API is unstable and subject to
frequent changes. Production use is not recommended until version **0.1.0**.


# 📜 License

All code in this project is dual-licensed under either:

  * MIT License
    ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

  * Apache License, Version 2.0,
    ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer! This dual-licensing approach is
the de-facto standard in the Rust ecosystem.
