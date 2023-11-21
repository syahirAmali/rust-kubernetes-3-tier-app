extern crate diesel;
extern crate rocket;

use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::consumer::{StreamConsumer, Consumer};
use rdkafka::message::Message;
use rdkafka::util::Timeout;

use futures::StreamExt;
use crate::schema::counter::dsl::counter;
use crate::schema::counter::dsl::count;
use crate::schema::config::dsl::config;
use crate::schema::config::dsl::text_string;

use diesel::prelude::*;
use dotenv::dotenv;
use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{get, post};
use std::env;
use super::schema::counter as other_counter;
use super::schema::config as other_config;

pub fn establish_connection_pg() -> (PgConnection, FutureProducer, StreamConsumer) {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let kafka_servers = env::var("KAFKA_SERVERS").expect("KAFKA_SERVERS must be set");

    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    // Create a Kafka producer and consumer
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", &kafka_servers)
        .create()
        .expect("Producer creation error");

    let consumer: StreamConsumer = ClientConfig::new()
        .set("bootstrap.servers", &kafka_servers)
        .create()
        .expect("Consumer creation error");

    (connection, producer, consumer)
}

pub async fn start_consumer() -> Result<(), Box<dyn std::error::Error>> {
    let (_, _, consumer) = establish_connection_pg();

    consumer.subscribe(&["counter_update", "config_update"]).expect("Can't subscribe to specified topic");

    let mut message_stream = consumer.stream();
    while let Some(message) = message_stream.next().await {
        match message {
            Err(_) => println!("Error while reading from stream."),
            Ok(m) => {
                let key = match m.key_view::<str>() {
                    None => "".to_string(),
                    Some(Ok(s)) => s.to_string(),
                    Some(Err(_)) => "".to_string(),
                };
                let payload = match m.payload_view::<str>() {
                    None => "".to_string(),
                    Some(Ok(s)) => s.to_string(),
                    Some(Err(_)) => "".to_string(),
                };
                println!("key: '{}', payload: '{}', topic: {}, partition: {}, offset: {}",
                         key, payload, m.topic(), m.partition(), m.offset());
            },
        };
    }

    Ok(())
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = other_counter)]
pub struct CounterStruct {
    pub id: i32,
    pub count: i32,
}

#[post("/counter/increment")]
pub fn increment_counter() -> Result<Json<CounterStruct>, Status> {
    let (mut connection, producer, _)  = establish_connection_pg();

    let counter_struct: CounterStruct = match counter.first::<CounterStruct>(&mut connection) {
        Ok(counter_result) => counter_result,
        Err(_) => {
            let new_counter = CounterStruct { id: 1, count: 0 };
            diesel::insert_into(counter).values(&new_counter).execute(&mut connection).unwrap();
            new_counter
        },
    };

    diesel::update(counter.find(counter_struct.id))
        .set(count.eq(counter_struct.count + 1))
        .execute(&mut connection).unwrap();

    let producer_send = producer.send(
        FutureRecord::<(), str>::to("counter_update")
            .payload("Counter incremented"),
        Timeout::Never,
    );

    if let Err(e) = futures::executor::block_on(producer_send) {
        eprintln!("Error delivering message: {:?}", e);
    }

    Ok(Json(counter_struct))
}

#[get("/counter")]
pub fn get_counter() -> Result<Json<CounterStruct>, Status> {
    let (mut connection, _, _)  = establish_connection_pg();

    match counter.first::<CounterStruct>(&mut connection) {
        Ok(counter_result) => Ok(Json(counter_result)),
        Err(_) => {
            let new_counter = CounterStruct { id: 1, count: 0 };
            diesel::insert_into(counter).values(&new_counter).execute(&mut connection).unwrap();
            Ok(Json(new_counter))
        },
    }
}

#[derive(Serialize, Deserialize, Queryable, Insertable, Debug)]
#[diesel(table_name = other_config)]
pub struct ConfigStruct {
    pub id: i32,
    pub text_string: String,
    pub user_role: String,
}

#[post("/config/update", format = "application/json", data = "<config_struct>")]
pub fn update_config(config_struct: Json<ConfigStruct>) -> Result<Json<ConfigStruct>, Status> {

    // Check if the user has the necessary permissions
    if config_struct.user_role != "admin" {
        return Err(Status::Forbidden);
    }

    let (mut connection, producer, _)  = establish_connection_pg();

    diesel::update(config.find(config_struct.id))
        .set(text_string.eq(&config_struct.text_string))
        .execute(&mut connection).unwrap();

    let producer_send = producer.send(
        FutureRecord::<(), str>::to("config_update")
            .payload("Config updated"),
        Timeout::Never,
    );

    if let Err(e) = futures::executor::block_on(producer_send) {
        eprintln!("Error delivering message: {:?}", e);
    }

    Ok(config_struct)
}

#[get("/config")]
pub fn get_config() -> Result<Json<ConfigStruct>, Status> {
    let (mut connection, _, _)  = establish_connection_pg();

    match config.first::<ConfigStruct>(&mut connection) {
        Ok(config_result) => Ok(Json(config_result)),
        Err(_) => {
            let new_config = ConfigStruct { id: 1, text_string: "Hello World".to_string(), user_role: "admin".to_string() };
            diesel::insert_into(config).values(&new_config).execute(&mut connection).unwrap();
            Ok(Json(new_config))
        },
    }
}