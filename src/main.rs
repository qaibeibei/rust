// use member_server::member;
// use member_server::new_member;
// use async_std::task::{sleep, spawn};
use dzq;
use member_server::new_member2;
use simple_poolweb::web::web_init;
// use std::time::Duration;

// use std::time::Instant;

#[async_std::main]
async fn main() {
    // let now = Instant::now();
    // println!("{:#?}",now);

    // member::init();
    // new_member2::init();
    // dzq::init();
    // for i in 1..10000 {
    //     println!("Hello, world! {}", i);
    // }
    web_init();
}
