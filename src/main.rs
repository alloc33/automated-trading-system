use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Build our application with a single route.
    let app = axum::Router::new().route("/",
        axum::routing::get(|| async { "Hello, World!" }));

    // Run our application as a hyper server on http://localhost:3000.
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
