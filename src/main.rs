use std::net::SocketAddr;
use hyper::server::Server;


mod db;
mod routes;
mod models;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let pool = db::init_db().await;

    let app = routes::create_routes().with_state(pool);
    // let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));


    println!("🚀 Server running on http://localhost:3000");
    // Server::builder(listener_stream).serve(app.into_make_service()).await.unwrap();
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
