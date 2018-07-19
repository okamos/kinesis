#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate lazy_static;
extern crate rand;
#[macro_use]
extern crate serde_json;
extern crate rocket;
extern crate rusoto_core;
extern crate rusoto_firehose;

use std::time::{SystemTime, UNIX_EPOCH};
use rand::{thread_rng, Rng};
use rand::distributions::{Alphanumeric};
use rusoto_core::{Region};
use rusoto_firehose::{
    KinesisFirehose,
    KinesisFirehoseClient,
    PutRecordInput,
    Record
};

lazy_static! {
    static ref CLIENT: KinesisFirehoseClient = KinesisFirehoseClient::simple(Region::ApNortheast1);
}
fn now() -> u64 {
    let now = SystemTime::now();
    let epoch = now.duration_since(UNIX_EPOCH).expect("Time wnt backwards");
    epoch.as_secs()
}

fn generate_session_id(n: usize) -> String {
    let mut rng = thread_rng();
    // TODO:  URL safe string are A-Z a-z 0-9 - _, . ~ is limited.
    rng.sample_iter(&Alphanumeric).take(n).collect()
}

#[get("/<wid>")]
fn index(wid: u32) -> String {
    let mut input = PutRecordInput::default();
    let mut record = Record::default();
    input.delivery_stream_name = String::from("your-stream-name");
    let event = json!({
        "wsid": generate_session_id(32),
        "usid": "",
        "eid": thread_rng().gen_range(1, 101),
        "wid": wid,
        "mid": thread_rng().gen_range(1, 10001),
        "pid": thread_rng().gen_range(1, 100001),
        "ts": now()
    });
    // TODO: error handling
    record.data = serde_json::to_vec(&event).unwrap();
    record.data.push(10); // line feed
    input.record = record;
    let _result = CLIENT.put_record(&input).sync().unwrap();
    // println("{:?}", result)
    format!("{}, we need to talk about your coolness.", event)
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
