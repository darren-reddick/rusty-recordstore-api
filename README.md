# rusty-recordstore-api

## Overview

This is an example project using warp and tokio to create an API for searching the stock of a record store (as in music record store!!).

<p align="left">
<img width="100" height="100" src="/assets/images/index.jpg">
</p>

## Prerequisites

Make sure you have installed all of the following prerequisites on your development machine:

* Git - [Download & Install](https://git-scm.com/downloads) Git. 
* rust - [Download & Install](https://doc.rust-lang.org/cargo/getting-started/installation.html) rust including cargo.

## Quick start

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

Test with a quick curl command to upload a new record
```
curl localhost:3030/record -X POST --data '{"title": "testing","artist": "The test band","format": "vinyl","year": 2021}' -H "Content-type: application/json"
```

If the test response looks like the below then we are good!:
```
{"uuid":"d1d59099-47eb-4592-949a-1323924c40f6","title":"testing","artist":"The test band","format":"vinyl","year":2021}
```

## Tests

### Unit Tests

Unit tests are defined within the same files as the functions they are testing.

### Integration Tests

Integration tests are defined within the **tests/** directory.

### Running tests

Tests can be run using cargo
```
cargo t
```


## Crate structure

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





