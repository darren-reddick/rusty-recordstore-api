use std::convert::Infallible;
use warp::{self, Filter};

use crate::handlers;
use crate::models;

/// All item routes
pub fn item_routes(
    db: models::SafeDB,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    get_item(db.clone())
        .or(post_item(db.clone()))
        .or(get_items(db.clone()))
        .or(delete_item(db.clone()))
        .or(update_item(db.clone()))
}

/// GET /item
fn get_items(
    db: models::SafeDB,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("item")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::get_items)
}

/// GET /item/:uuid
fn get_item(
    db: models::SafeDB,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("item" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::get_item)
}

/// DELETE /item/:uuid
fn delete_item(
    db: models::SafeDB,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("item" / String)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handlers::delete_item)
}

/// POST /item
fn post_item(
    db: models::SafeDB,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path("item")
        .and(warp::post())
        .and(with_db(db))
        .and(json_body())
        .and_then(handlers::add_item)
}

/// PUT /people/:uuid
fn update_item(
    db: models::SafeDB,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("item" / String)
        .and(warp::put())
        .and(with_db(db))
        .and(json_body())
        .and_then(handlers::update_item)
}

fn with_db(
    db: models::SafeDB,
) -> impl Filter<Extract = (models::SafeDB,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (models::Item,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::StatusCode;

    #[tokio::test]
    async fn test_get_empty() {
        let filter = get_items(models::inmemdb::init_db(None));

        let value = warp::test::request()
            .path("/item/")
            .method("GET")
            .reply(&filter)
            .await;
        assert_eq!(value.body(), "[]");
        assert_eq!(value.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_nonexist() {
        let filter = get_item(models::inmemdb::init_db(None));

        let value = warp::test::request()
            .path("/item/xxxx-xxxx-xxxx")
            .method("GET")
            .reply(&filter)
            .await;
        assert_eq!(value.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_get_people() {
        let db = models::inmemdb::init_db(Some(vec![models::Item::new(
            "Spastik".to_string(),
            "Plastikman".to_string(),
            "vinyl".to_string(),
            1994,
        )]));
        let filter = get_items(db);

        let res = warp::test::request()
            .path("/item/")
            .method("GET")
            .reply(&filter)
            .await;

        let people: Vec<models::Item> = serde_json::from_slice(res.body()).unwrap();
        assert_eq!(people[0].artist, "Plastikman");

        assert_eq!(res.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_add_person() {
        let db = models::inmemdb::init_db(None);
        let filter = post_item(db);

        let res = warp::test::request()
            .path("/item/")
            .method("POST")
            .json(&models::Item::new(
                "Xpander".to_string(),
                "Sasha".to_string(),
                "cd".to_string(),
                1995,
            ))
            .reply(&filter)
            .await;

        assert_eq!(res.status(), StatusCode::OK);

        let item: models::Item = serde_json::from_slice(res.body()).unwrap();

        assert_eq!(item.title, "Xpander")
    }
}
