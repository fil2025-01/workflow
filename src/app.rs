use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/workflow.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=move |_| { alert("Hello World!"); }>
            "Click Me"
        </button>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the client
    // will simply render the component without the appropriate HTTP header
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // use leptos_axum::ResponseOptions;
        // use leptos_axum::ResponseParts;
        // let response = use_context::<ResponseOptions>();
        // if let Some(response) = response {
        //     response.set_status(leptos_axum::http::StatusCode::NOT_FOUND);
        // }
    }

    view! {
        <h1>"Not Found"</h1>
    }
}

#[cfg(not(feature = "ssr"))]
fn alert(msg: &str) {
    let _ = web_sys::window().unwrap().alert_with_message(msg);
}

#[cfg(feature = "ssr")]
fn alert(_msg: &str) {
    // No-op on server
}
