use embedded_svc::httpd::*;
use embedded_svc::mqtt::client::utils::ConnState;
use embedded_svc::mqtt::client::{Client, Connection, MessageImpl, Publish, QoS};
use esp_idf_svc::mqtt::client::*;
use esp_idf_sys::{esp, EspError};
use std::{cell::RefCell, env, sync::atomic::*, sync::Arc, thread, time::*};

// const MQTT_HOST: &str = env!("RUST_ESP32_MQTT_HOST");
// const MQTT_PORT: &str = env!("RUST_ESP32_MQTT_PORT");

// pub fn mqtt_init() {
//     let mqtt_client = start_mqtt_client().unwrap();
// }

// fn start_mqtt_client() -> Result<EspMqttClient<ConnState<MessageImpl, EspError>>> {
//     println!("About to start MQTT client");

//     let conf = MqttClientConfiguration {
//         client_id: Some("rust-esp32-std-demo"),
//         crt_bundle_attach: Some(esp_idf_sys::esp_crt_bundle_attach),

//         ..Default::default()
//     };

//     let mqtt_url = format!("mqtt://{}:{}", MQTT_HOST, MQTT_PORT);
//     let (mut client, mut connection) = EspMqttClient::new_with_conn(mqtt_url, &conf)?;

//     println!("MQTT client started");

//     // Need to immediately start pumping the connection for messages, or else subscribe() and publish() below will not work
//     // Note that when using the alternative constructor - `EspMqttClient::new` - you don't need to
//     // spawn a new thread, as the messages will be pumped with a backpressure into the callback you provide.
//     // Yet, you still need to efficiently process each message in the callback without blocking for too long.
//     //
//     // Note also that if you go to http://tools.emqx.io/ and then connect and send a message to topic
//     // "rust-esp32-std-demo", the client configured here should receive it.
//     thread::spawn(move || {
//         println!("MQTT Listening for messages");

//         while let Some(msg) = connection.next() {
//             match msg {
//                 Err(e) => println!("MQTT Message ERROR: {}", e),
//                 Ok(msg) => println!("MQTT Message: {:?}", msg),
//             }
//         }

//         println!("MQTT connection loop exit");
//     });

//     client.subscribe("rust-esp32-std-demo", QoS::AtMostOnce)?;

//     println!("Subscribed to all topics (rust-esp32-std-demo)");

//     client.publish(
//         "rust-esp32-std-demo",
//         QoS::AtMostOnce,
//         false,
//         "Hello from rust-esp32-std-demo!".as_bytes(),
//     )?;

//     println!("Published a hello message to topic \"rust-esp32-std-demo\"");

//     Ok(client)
// }

// // client_id needs to be unique
// pub fn mqtt_init_2() {
//     let conf = MqttClientConfiguration {
//         client_id: Some("esp32-temperature-logger"),
//         keep_alive_interval: Some(Duration::from_secs(120)),
//         ..Default::default()
//     };

//     let (mut client, mut connection) = EspMqttClient::new("test1234@broker.emqx.io:8083", &conf)?;

//     thread::spawn(move || {
//         println!("MQTT Listening for messages");

//         while let Some(msg) = connection.next() {
//             match msg {
//                 Err(e) => println!("MQTT Message ERROR: {}", e),
//                 Ok(msg) => println!("MQTT Message: {:?}", msg),
//             }
//         }

//         println!("MQTT connection loop exit");
//     });

//     println!("Connected to MQTT");
// }
