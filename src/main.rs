mod app;
mod run;
mod vertex;
mod widgets;
use crate::run::run;

#[tokio::main]
async fn main() {
    run().await.unwrap();
}
