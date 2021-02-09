use std::convert::Infallible;
use warp::{self, Filter};

use crate::handlers;
use crate::models;

/// All record routes
pub fn record_routes(
  db: models::SafeDB,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
  get_record(db.clone())
    .or(post_record(db.clone()))
    .or(get_records(db.clone()))
    .or(delete_record(db.clone()))
    .or(update_record(db.clone()))
}

/// GET /record
fn get_records(
  db: models::SafeDB,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
  warp::path("record")
    .and(warp::get())
    .and(with_db(db))
    .and_then(handlers::get_records)
}

/// GET /record/:uuid
fn get_record(
  db: models::SafeDB,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
  warp::path!("record" / String)
    .and(warp::get())
    .and(with_db(db))
    .and_then(handlers::get_record)
}

/// DELETE /record/:uuid
fn delete_record(
  db: models::SafeDB,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
  warp::path!("record" / String)
    .and(warp::delete())
    .and(with_db(db))
    .and_then(handlers::delete_record)
}

/// POST /record
fn post_record(
  db: models::SafeDB,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
  warp::path("record")
    .and(warp::post())
    .and(with_db(db))
    .and(json_body())
    .and_then(handlers::add_record)
}

/// PUT /people/:uuid
fn update_record(
  db: models::SafeDB,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
  warp::path!("record" / String)
    .and(warp::put())
    .and(with_db(db))
    .and(json_body())
    .and_then(handlers::update_record)
}

fn with_db(
  db: models::SafeDB,
) -> impl Filter<Extract = (models::SafeDB,), Error = Infallible> + Clone {
  warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (models::Record,), Error = warp::Rejection> + Clone {
  warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[cfg(test)]
mod tests {
  use super::*;
  use http::StatusCode;

  #[tokio::test]
  async fn test_get_empty() {
    let filter = get_records(models::inmemdb::init_db(None));

    // Execute `sum` and get the `Extract` back.
    let value = warp::test::request()
      .path("/record/")
      .method("GET")
      .reply(&filter)
      .await;
    assert_eq!(value.body(), "[]");
    assert_eq!(value.status(), StatusCode::OK);
  }

  #[tokio::test]
  async fn test_get_people() {
    let db = models::inmemdb::init_db(Some(vec![models::Record::new(
      "Spastik".to_string(),
      "Plastikman".to_string(),
      "vinyl".to_string(),
      1994,
    )]));
    let filter = get_records(db);

    let res = warp::test::request()
      .path("/record/")
      .method("GET")
      .reply(&filter)
      .await;

    let people: Vec<models::Record> = serde_json::from_slice(res.body()).unwrap();
    assert_eq!(people[0].artist, "Plastikman");

    assert_eq!(res.status(), StatusCode::OK);
  }

  #[tokio::test]
  async fn test_add_person() {
    let db = models::inmemdb::init_db(None);
    let filter = post_record(db);

    let res = warp::test::request()
      .path("/record/")
      .method("POST")
      .json(&models::Record::new(
        "Xpander".to_string(),
        "Sasha".to_string(),
        "cd".to_string(),
        1995,
      ))
      .reply(&filter)
      .await;

    assert_eq!(res.status(), StatusCode::OK);

    let record: models::Record = serde_json::from_slice(res.body()).unwrap();

    assert_eq!(record.title, "Xpander")
  }
}
