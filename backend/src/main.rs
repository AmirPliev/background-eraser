use axum::Router;
mod detect;
mod image_proc;
mod api;

#[tokio::main]
async fn main() {
    let app: Router = api::create_api();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:5555").await.unwrap();
    println!("I am hosting the API now!");
    axum::serve(listener, app).await.unwrap();
}

