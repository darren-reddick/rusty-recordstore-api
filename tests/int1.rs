extern crate storelib;

use reqwest;
use storelib::models;
use storelib::routes::item_routes;
use tokio::sync::oneshot;

#[tokio::test(flavor = "multi_thread")]
async fn simple_get_status_ok() {
    pretty_env_logger::init();

    let (tx, rx) = oneshot::channel();

    let db = models::inmemdb::init_db(None);

    let item_routes = item_routes(db);

    let (addr, server) =
        warp::serve(item_routes).bind_with_graceful_shutdown(([127, 0, 0, 1], 3030), async {
            rx.await.ok();
        });

    tokio::spawn(server);

    let res = reqwest::get(&format!("http://{}/item", addr)).await;

    assert_eq!(res.unwrap().status(), reqwest::StatusCode::OK);

    let _ = tx.send(());
}
