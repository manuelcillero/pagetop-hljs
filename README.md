**HighlightJS** (`pagetop-hljs`) is a PageTop module that displays beautiful code snippets on web
pages using the versatile [highlight.js](https://highlightjs.org/) JavaScript library.

  * Supports **90+** coding languages.
  * Choose from all **95+** available themes.
  * Provides a component for adding code snippets.
  * Highlight multi-line blocks of code.
  * Detects `language-` and `lang-` class prefixes.
  * Customize the *highlight.js* init JavaScript.
  * Smart loading of CSS & JS assets.

## Usage

Add to `Cargo.toml` dependency to `pagetop-hljs`:

```rust
[dependencies]
pagetop-hljs = "<Version>"
```

Add the dependency to `pagetop_hljs::HighlightJS` in the module that uses it:

```rust
use pagetop::prelude::*;

impl ModuleTrait for ModuleName {
    // ...
    fn dependencies(&self) -> Vec<ModuleStaticRef> {
        vec![
            // ...
            &pagetop_hljs::HighlightJS
            // ...
        ]
    }
    // ...
}
```

Now your module can add code snippets on web pages:

```rust
use pagetop_hljs::prelude::*;

#[service::get("/")]
async fn hljs_sample(request: service::HttpRequest) -> ResultPage<Markup, FatalError> {
    Page::new(request)
        .with_in(
            "content",
            Snippet::with(
                HljsLang::Rust,
                r##"
async fn hello_world(request: service::HttpRequest) -> ResultPage<Markup, FatalError> {
    Page::new(request)
        .with_in("content", Html::with(html! { h1 { "Hello World!" } }))
        .render()
}
                "##,
            ),
        )
        .render()
}
```

## Note

HighlightJS uses [`ActionBeforeRenderPage`](pagetop::response::page::ActionBeforeRenderPage) with a
weight of 99 to add page assets. If you use this action to alter HighlightJS rendering, such as
specifying the theme for snippets, please ensure that your action has a weight lower than 99.
Default 0 is ok.


# 📦 About PageTop

[PageTop](https://github.com/manuelcillero/pagetop/tree/main/pagetop) is an opinionated web
development framework that uses some of the most stable and popular Rust packages to build modular,
extensible and configurable Server-Side Rendering (SSR) solutions.


# 🚧 Warning

**PageTop** framework is currently in active development. The API is unstable and subject to
frequent changes. Production use is not recommended until version **0.1.0**.


# 📜 License

This project is licensed under either of the following licenses, at your option:

  * Apache License, Version 2.0,
    ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)

  * MIT License
    ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)
