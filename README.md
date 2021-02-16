# rusty-recordstore-api

<p align="left">
<img src="https://img.shields.io/github/v/release/darren-reddick/rusty-recordstore-api">
<img src="https://github.com/darren-reddick/rusty-recordstore-api/workflows/Continuous%20integration/badge.svg">
<img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg">
</p>

This is an example web api project using warp and tokio to create an API for managing the stock of a recordstore (as in music record store!!).

<p align="left">
<img width="100" height="100" src="/assets/images/index.jpg">
</p>

## Overview

The project was created as a way to get more experience programming in rust through a simple, small web application.

The features it touches upon include:
* async / await (with the tokio runtime)
* integration tests for async applications (not as straightfoward as first thought!)
* unit tests
* using traits to abstract the database (very handy for testing)

## Usage

### Prerequisites

Make sure you have installed all of the following prerequisites on your development machine:

* Git - [Download & Install](https://git-scm.com/downloads) Git. 
* rust - [Download & Install](https://doc.rust-lang.org/cargo/getting-started/installation.html) rust including cargo.

### Quick start

Clone the gitHub repository

```
git clone https://github.com/darren-reddick/rusty-recordstore-api.git
```

Build the binary

```
cd rusty-recordstore-api/
cargo b
```

Run the app
```
cargo run
```

Test with a quick curl command to upload a new item
```
curl localhost:3030/item -X POST --data '{"title": "testing","artist": "The test band","format": "vinyl","year": 2021}' -H "Content-type: application/json"
```

If the test response looks like the below then we are good!
```
{"uuid":"d1d59099-47eb-4592-949a-1323924c40f6","title":"testing","artist":"The test band","format":"vinyl","year":2021}
```

### Tests

#### Unit Tests

Unit tests are defined alongside the functions they are testing.

#### Integration Tests

Integration tests are defined within the **tests/** directory and use the **tokio:test** runtime.

#### Running tests

Tests can be run using cargo
```
cargo t
```

## API

The following endpoints and actions are supported by the API.


| HTTP Verb | Path             | Request Content-Type | Request body | Response Content-Type | Example response body |
|-----------|------------------|----------------------|--------------|-----------------------|-----------------------|
| GET       | `/item`        | `application/json`   | -            | `application/json`    | `[ {"uuid":"f2af50d9-8a1a-4622-85f6-44680c31f34b","title":"testing","artist":"The test band","format":"vinyl","year":2021}, ... ]` |
| POST      | `/item`        | `application/json`   | `{"title":"Selected Ambient Works Vol.1","artist":"Aphex Twin","format":"vinyl","year":1992}` | `application/json`    |  `{"uuid":"fb23942b-0e3f-4665-8164-9a4ec883edbf","title":"Selected Ambient Works Vol.1","artist":"Aphex Twin","format":"vinyl","year":1992}` |
| GET       | `/item/{uuid}` | `application/json`   | -            | `application/json`    | `{"uuid":"fb23942b-0e3f-4665-8164-9a4ec883edbf","title":"Selected Ambient Works Vol.1","artist":"Aphex Twin","format":"vinyl","year":1992}` |
| DELETE    | `/item/{uuid}` | `application/json`   | -            | `application/json`    | - |
| PUT       | `/people/{uuid}` | `application/json`   | `{"uuid":"fb23942b-0e3f-4665-8164-9a4ec883edbf","title":"Selected Ambient Works Vol.1","artist":"Aphex Twin","format":"cassette","year":1992}` | `application/json`    | - |


## Crates Layout

```
.
├── storelib
│   ├── handlers
│   │   └── tests
│   ├── routes
│   │   └── tests
│   └── models
│       ├── inmemdb
│       └── tests
├── storebin
└── int1
```




## License

Licensed under 

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)





