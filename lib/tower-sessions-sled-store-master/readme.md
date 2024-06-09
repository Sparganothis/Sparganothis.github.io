<h1 align="center">
    tower-sessions-sled-store
</h1>

<p align="center">
    A tower-sessions backend for Sled
</p>

## Overview

This create provides an Sled back-end for the tower-session session management crate.

### Note on stability

The sled library has not yet released their 1.0 release. According to the sled developers there will be an overhaul of the on disk format for version 1.0 which may require additional efforts to migrate data.

### Note on performance

While sled performs fairly well for general tasks, deletion of expired records may cause slowdowns when using sled as your session store.

As sled is a simple key-value database checking the expiration for every key requires iterating every key in the database. This may cause performance issues with high numbers of active sessions

## Usage

Creating a `SledStore` only requires a sled `Tree`

### Usage example

```rust
use std::net::SocketAddr;

use axum::{response::IntoResponse, routing::get, Router};
use serde::{Deserialize, Serialize};
use time::Duration;
use tower_sessions::{Expiry, Session, SessionManagerLayer};
use tower_sessions_sled_store::SledStore;

const COUNTER_KEY: &str = "counter";

#[derive(Default, Deserialize, Serialize)]
struct Counter(usize);

async fn handler(session: Session) -> impl IntoResponse {
    let counter: Counter = session.get(COUNTER_KEY).await.unwrap().unwrap_or_default();
    session.insert(COUNTER_KEY, counter.0 + 1).await.unwrap();
    format!("Current count: {}", counter.0)
}

#[tokio::main]
async fn main() {
    // Open a new sled database
    let sled = sled::open("storage").unwrap();

    // Open a tree named session within that database
    let session_store = SledStore::new(sled.open_tree("session").unwrap());
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(10)));

    let app = Router::new().route("/", get(handler)).layer(session_layer);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
```
