#[macro_use] extern crate rocket;

mod services;
mod models;
mod schema;

use std::net::IpAddr;
use std::net::Ipv4Addr;

use rocket::Config;
use tokio;
use rocket::fs::FileServer;
use services::increment_counter;
use services::get_counter;
use services::get_config;
use services::update_config;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .configure(Config {
            address: IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)),
            ..Config::default()
        })
        .mount("/", FileServer::from("/app/src/static"))
        .mount("/api", routes![increment_counter, get_counter, get_config, update_config])
        .manage(async {
            tokio::task::spawn_blocking(|| {
                tokio::runtime::Runtime::new().unwrap().block_on(async {
                    if let Err(e) = services::start_consumer().await {
                        eprintln!("Error starting consumer: {}", e);
                    }
                });
            }).await.unwrap();
        })
}