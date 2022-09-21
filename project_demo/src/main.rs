#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use std::env;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;

mod error_handler;
mod schema;
mod db;
mod employees;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Rust 服务要启动了");
    dotenv().ok();
    db::init();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().configure(employees::init_routes));

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("HOST有误");
            let port = env::var("PORT").expect("PORT有误");
            println!("启动在{}:{}", host, port);
            server.bind(format!("{}:{}", host, port))?
        }
    };

    server.run().await
}
