use pagetop::prelude::*;
use pagetop_hljs::prelude::*;

new_handle!(APP_HLJS_SAMPLE);

struct HljsSample;

impl ModuleTrait for HljsSample {
    fn handle(&self) -> Handle {
        APP_HLJS_SAMPLE
    }

    fn dependencies(&self) -> Vec<ModuleRef> {
        vec![&pagetop_hljs::HighlightJS]
    }

    fn actions(&self) -> Vec<Action> {
        vec![action!(ActionAfterPrepareBody => after_prepare_body)]
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        cfg.route("/", service::web::get().to(hljs_sample));
    }
}

async fn hljs_sample(request: service::HttpRequest) -> ResultPage<Markup, FatalError> {
    Page::new(request)
        .with_in(
            "content",
            Snippet::with(
                HljsLang::Rust,
                r###"
use pagetop::prelude::*;

new_handle!(APP_HELLO_WORLD);

struct HelloWorld;

impl ModuleTrait for HelloWorld {
    fn handle(&self) -> Handle {
        APP_HELLO_WORLD
    }

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        cfg.route("/", service::web::get().to(hello_world));
    }
}

async fn hello_world(request: service::HttpRequest) -> ResultPage<Markup, FatalError> {
    Page::new(request)
        .with_in("content", Html::with(html! { h1 { "Hello World!" } }))
        .render()
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&HelloWorld).unwrap().run()?.await
}
                "###,
            ),
        )
        .render()
}

fn after_prepare_body(page: &mut Page) {
    HighlightJS.set_theme(HljsTheme::Sunburst, page.context());
}

#[pagetop::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&HljsSample).unwrap().run()?.await
}
