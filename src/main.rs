extern crate storelib;

use storelib::models::inmemdb::init_db;
use storelib::routes::item_routes;

fn main() {
    run()
}

#[tokio::main]
async fn run() {
    pretty_env_logger::init();

    let db = init_db(None);

    let item_routes = item_routes(db);

    warp::serve(item_routes).run(([127, 0, 0, 1], 3030)).await;
}
