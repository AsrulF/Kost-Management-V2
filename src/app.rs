use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

use crate::features::auth::login_form::LoginForm;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link href = "./style/output.css" rel = "stylesheet" />
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/kost-management-v2.css"/>

        // sets the document title
        <Title text="Welcome to Kost Management"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {

    view! {
        <div
            class = "min-h-screen bg-cover bg-center flex items-center justify-end"
            style:background-image = "url('/images/homepage-bg.jpg')"
        >
            <div class = "absolute inset-0 bg-black/40"/>
            <div
                class = "relative z-10 h-full flex items-center pr-20"
            >
                <div
                    class = "bg-white/90 backdrop-blur p-8 rounded-xl shadow-xl"
                >
                    <LoginForm />
                </div>
            </div>
        </div>  
    }
}
