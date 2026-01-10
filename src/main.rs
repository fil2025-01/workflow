use axum::{
    response::Html,
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // Build our application with a route
    let app = Router::new().route("/", get(handler));

    // Define the address to listen on
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("Listening on http://127.0.0.1:3000");

    // Run the server
    axum::serve(listener, app).await.unwrap();
}

// Handler that returns HTML
async fn handler() -> Html<&'static str> {
    Html(include_str!("../index.html"))

}
