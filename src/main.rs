use nats;
use serde_json::{json};
use serde::{Serialize, Deserialize};
use std::time::{SystemTime};
use std::{time, thread};

#[derive(Serialize, Deserialize, Debug)]
struct GarageSensorState {
    distance: f64,
    time: u64
}

fn post_update(conn: &nats::Connection, distance: f64) {
    let payload  = GarageSensorState {
        distance: distance,
        time: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    };

    let payload_json = json!(payload);

    conn.publish("foo", payload_json.to_string());
}

fn main() {
    let nc = nats::connect("167.99.232.215:4222").unwrap();

    loop {
        let ten_millis = time::Duration::from_millis(1_000);
        let now = time::Instant::now();

        thread::sleep(ten_millis);
        println!("posting update");
        post_update(&nc, 10.3)
    }

    post_update(&nc, 10.3)
}
