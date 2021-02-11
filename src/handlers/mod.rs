use std::convert::Infallible;

use crate::models;

pub async fn get_items(db: models::SafeDB) -> Result<impl warp::Reply, Infallible> {
    let c = db.lock().await;
    let c = &*c;
    Ok(warp::reply::json(&c.get_items()))
}

pub async fn get_item(uuid: String, db: models::SafeDB) -> Result<impl warp::Reply, Infallible> {
    let c = db.lock().await;
    let c = &*c;
    Ok(warp::reply::json(&c.get_item(uuid)))
}

pub async fn delete_item(uuid: String, db: models::SafeDB) -> Result<impl warp::Reply, Infallible> {
    let c = &mut db.lock().await;
    let c = &mut *c;
    c.delete_item(uuid).unwrap();
    Ok(warp::reply::json(&{}))
}

pub async fn add_item(
    db: models::SafeDB,
    item: models::Item,
) -> Result<impl warp::Reply, Infallible> {
    let c = &mut db.lock().await;
    let c = &mut *c;
    Ok(warp::reply::json(&c.add_item(item)))
}

pub async fn update_item(
    uuid: String,
    db: models::SafeDB,
    item: models::Item,
) -> Result<impl warp::Reply, Infallible> {
    let c = &mut db.lock().await;
    let c = &mut *c;
    c.update_item(uuid, item).unwrap();
    Ok(warp::reply::json(&{}))
}
