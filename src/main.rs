pub mod proxy;
pub mod mongo;
pub mod proxy_call;
#[macro_use]
extern crate lazy_static;

use async_std::task;

#[tokio::main]
async fn main() {
    proxy::proxy_call().await;

}