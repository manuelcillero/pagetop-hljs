<div align="center">

  <h1>PageTop HighlightJS</h1>

  [![crates.io](https://img.shields.io/crates/v/pagetop-hljs.svg)](https://crates.io/crates/pagetop-hljs)
  [![docs.rs](https://docs.rs/pagetop-hljs/badge.svg)](https://docs.rs/pagetop-hljs)
  [![Dependencies](https://deps.rs/crate/pagetop-hljs/0.0.2/status.svg)](https://deps.rs/crate/pagetop-hljs/0.0.2)
  [![MIT or Apache 2.0 licensed](https://img.shields.io/crates/l/pagetop-hljs.svg)](#license)

</div>

---

**HighlightJS** (`pagetop-hljs`) is a PageTop module that displays beautiful code snippets on web
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

impl ModuleTrait for MyModule {
    // ...
    fn dependencies(&self) -> Vec<ModuleRef> {
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
async fn hljs_sample(request: service::HttpRequest) -> ResultPage<Markup, FatalError> {
    Page::new(request)
        .with_in(
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

[PageTop](https://github.com/manuelcillero/pagetop/tree/main/pagetop) is an opinionated Rust web
development framework to build secure and modular Server-Side Rendering (SSR) web solutions.


# 🚧 Warning

**PageTop** framework is currently in active development. The API is unstable and subject to
frequent changes. Production use is not recommended until version **0.1.0**.


# 📜 License

This project is licensed under either of the following licenses, at your option:

  * Apache License, Version 2.0,
    ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)

  * MIT License
    ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)
