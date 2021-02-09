extern crate storelib;

use storelib::models::inmemdb::init_db;
use storelib::routes::record_routes;

fn main() {
    run()
}

#[tokio::main]
async fn run() {
    pretty_env_logger::init();

    let db = init_db(None);

    let record_routes = record_routes(db);

    warp::serve(record_routes).run(([127, 0, 0, 1], 3030)).await;
}
