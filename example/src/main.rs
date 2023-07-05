use pagetop::prelude::*;
use pagetop_hljs::prelude::*;

use_handle!(APP_HLJS_SAMPLE);

struct HljsSample;

impl ModuleTrait for HljsSample {
    fn handle(&self) -> Handle {
        APP_HLJS_SAMPLE
    }

    fn dependencies(&self) -> Vec<ModuleStaticRef> {
        vec![&pagetop_hljs::HighlightJS]
    }

    fn actions(&self) -> Vec<Action> {
        vec![action!(actions::page::ActionBeforeRenderPage => before_render_page)]
    }

    fn init(&self) {}

    fn configure_service(&self, cfg: &mut service::web::ServiceConfig) {
        cfg.service(hljs_sample);
    }
}

#[service::get("/")]
async fn hljs_sample(request: service::HttpRequest) -> ResultPage<Markup, FatalError> {
    Page::new(request)
        .with_in(
            "content",
            Snippet::with(
                HljsLang::Rust,
                r##"
use pagetop::prelude::*;

use_handle!(APP_HELLO_WORLD);

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&HelloWorld).unwrap().run()?.await
}
                "##,
            ),
        )
        .render()
}

fn before_render_page(page: &mut Page) {
    HighlightJS.enable_theme(HljsTheme::Sunburst, page.context());
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Application::prepare(&HljsSample).unwrap().run()?.await
}
