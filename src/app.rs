use crate::page::{homepage, new, outdated};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    SsrMode, StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html> 
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="sja" href="/pkg/sja.css" />
        <Title text="SJA" />
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("/") view=homepage::HomePage ssr=SsrMode::Async />
                    <Route
                        path=StaticSegment("/outdated")
                        view=outdated::OutdatedPage
                        ssr=SsrMode::Async
                    />
                    <Route path=StaticSegment("/new") view=new::NewPage ssr=SsrMode::Async />
                </Routes>
            </main>
        </Router>
    }
}
