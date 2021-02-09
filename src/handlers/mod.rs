use std::convert::Infallible;

use crate::models;

pub async fn get_records(db: models::SafeDB) -> Result<impl warp::Reply, Infallible> {
  let c = db.lock().await;
  let c = &*c;
  Ok(warp::reply::json(&c.get_records()))
}

pub async fn get_record(uuid: String, db: models::SafeDB) -> Result<impl warp::Reply, Infallible> {
  let c = db.lock().await;
  let c = &*c;
  Ok(warp::reply::json(&c.get_record(uuid)))
}

pub async fn delete_record(
  uuid: String,
  db: models::SafeDB,
) -> Result<impl warp::Reply, Infallible> {
  let c = &mut db.lock().await;
  let c = &mut *c;
  c.delete_record(uuid).unwrap();
  Ok(warp::reply::json(&{}))
}

pub async fn add_record(
  db: models::SafeDB,
  record: models::Record,
) -> Result<impl warp::Reply, Infallible> {
  let c = &mut db.lock().await;
  let c = &mut *c;
  Ok(warp::reply::json(&c.add_record(record)))
}

pub async fn update_record(
  uuid: String,
  db: models::SafeDB,
  record: models::Record,
) -> Result<impl warp::Reply, Infallible> {
  let c = &mut db.lock().await;
  let c = &mut *c;
  c.update_record(uuid, record).unwrap();
  Ok(warp::reply::json(&{}))
}
