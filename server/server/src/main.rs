use core::net::Ipv4Addr;

use axum::{Router, routing::post};
use server_macros::proj;

const PORT: u16 = proj!("PORT");

mod json_multipart;
mod upload;

#[tokio::main(flavor = "current_thread")]
async fn main() -> std::io::Result<()> {
    let app = Router::new().route("/upload", post(upload::upload));

    let listener = tokio::net::TcpListener::bind((Ipv4Addr::LOCALHOST, PORT)).await?;

    println!("openplanner server running @:");
    println!("{}:{}", Ipv4Addr::LOCALHOST, PORT);

    axum::serve(listener, app).await
}
