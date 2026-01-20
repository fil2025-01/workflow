use axum::{
    routing::{get, post, patch},
    Router,
    extract::{FromRef, State},
    response::IntoResponse,
};
use tower_http::services::ServeDir;
use tower::util::ServiceExt; // For oneshot
use sqlx::postgres::PgPool;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use workflow::app::*; // Import App from the library crate

use workflow::api::handlers::{
    handler,
    upload_handler,
    list_recordings,
    style_handler,
    script_handler,
    delete_recording,
    get_groups,
    update_recording
};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub leptos_options: LeptosOptions,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.db.clone()
    }
}

impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    // Database Connection
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&db_url).await.expect("Failed to connect to Postgres");

    // Leptos Config
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let state = AppState {
        db: pool,
        leptos_options: leptos_options.clone(),
    };

    let app = Router::new()
        // API Routes
        .route("/upload", post(upload_handler))
        .route("/recordings", get(list_recordings).delete(delete_recording))
        .route("/recordings/:id", patch(update_recording))
        .route("/groups", get(get_groups))

        // Static file serving for recordings
        .nest_service("/files", ServeDir::new("recordings"))

        // Legacy UI
        .route("/legacy", get(handler))
        .route("/style.css", get(style_handler))
        .route("/script.js", get(script_handler))

        // Leptos
        .leptos_routes(&state, routes, App)
        .fallback(file_and_error_handler)
        .with_state(state);

    println!("listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn file_and_error_handler(uri: axum::http::Uri, State(options): State<LeptosOptions>, req: axum::http::Request<axum::body::Body>) -> axum::response::Response {
    let root = options.site_root.clone();
    let res = get_static_file(uri.clone(), &root).await.unwrap();

    if res.status() == axum::http::StatusCode::OK {
        res.into_response()
    } else {
        let handler = leptos_axum::render_app_to_stream(options.to_owned(), App);
        handler(req).await.into_response()
    }
}

async fn get_static_file(uri: axum::http::Uri, root: &str) -> Result<axum::http::Response<axum::body::Body>, (axum::http::StatusCode, String)> {
    let req = axum::http::Request::builder().uri(uri.clone()).body(axum::body::Body::empty()).unwrap();
    // `ServeDir` implements `tower::Service` but we need to call it manually or use a proper static file handler.
    match ServeDir::new(root).oneshot(req).await {
        Ok(res) => {
            let (parts, body) = res.into_parts();
            let body = axum::body::Body::new(body);
            Ok(axum::http::Response::from_parts(parts, body))
        },
        Err(err) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", err),
        )),
    }
}
