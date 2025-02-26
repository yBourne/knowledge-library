use tokio::net::TcpListener;
use dotenvy::dotenv;
use hyper::server::Server;
use tokio_stream::wrappers::TcpListenerStream;

mod db;
mod routes;
mod models;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = db::init_db().await;

    let app = routes::create_routes().with_state(pool);
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    // Convert the TcpListener into a Hyper-compatible stream
    let listener_stream = TcpListenerStream::new(listener);


    println!("🚀 Server running on http://localhost:3000");
    Server::builder(listener_stream).serve(app.into_make_service()).await.unwrap();
}
