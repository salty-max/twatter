# Twatter

A cheap Twitter clone, in Rust

## Features

- [x] Create a post
  - [x] Limit the characters per post
  - [x] Optionally can be response to another post
  - [x] Optionally can be response to response
  - [x] Posts are stored in postgres
  - [x] Posts are validated before storage in database
  - [x] Cannot reply to a deleted post
- [x] Get a list of all top-level posts
  - [x] Text
  - [x] Likes
  - [x] Count of immediate children
  - [x] Deleted posts are ignored
- [x] Get one post
  - [x] Get immediate responses to the post
  - [x] Text
  - [x] Likes
  - [x] Deleted posts respond with a 404
- [x] Update post
  - [x] Text
  - [ ] Deleted posts respond with a 404
- [x] Delete post
  - [x] Soft delete post
  - [ ] Deleted posts respond with a 404

## Tech

- rust v1.74.1
  - with toolcahins
    - wasm32-unknown-unknown

### Client

- yew v0.21.0
  - with features
    - csr
- yew-router v0.18.0
- yewdux v0.10.0
- yew_icons v0.8.0
  - with features
    - lucide
- gloo v0.11.0
  - with features
    - console
    - net
- wasm-bindgen v0.2.89
- web-sys v0.3.66
  - with features
    - Element
    - EventTarget
    - HtmlElement
    - HtmlTextAreaElement
    - Node
- serde v1.0.193
  - with features
    - derive
- dotenvy_macro v0.15.7
- eyre v0.6.11
- cli
  - trunk v0.18.3

### API

- axum v0.7.2 
- dotenvy v0.15.7
- eyre v0.6.11
- tokio v1.35.0
  - with features
    - net
    - rt-multi-thread
    - macros
- serde v1.0.193
  - with features
    - derive
- serde_json v1.0.108
- sqlx v0.7.3
  - with features
    - postgres
    - runtime-tokio-rustls
- tracing v0.1.40
- tracing-subscriber v0.3.18
- tower-http v0.5.0
  - with features
    - trace
    - cors
- cli (use `cargo install`)
  - sqlx-cli v0.7.3

## Setup

1. Create the dotenv file by copying [.env.example](./.env.example) to `.env` and update it to your convenience.

```shell
cp .env.example .env
```

## Run

```shell
cargo run
```

## Database

A Docker compose file is included to run a Postgres database. Run this command to start the database.

```shell
docker compose up -d
```

### Connecting to the database locally

```sh
docker compose exec database psql -U postgres
```

### Models

#### Posts

| PK  | FK  | Name      | Type         | Nullable  | Default |
|-----|-----|-----------|--------------|-----------|---------|
|  *  |  *  | post_id   | serial       |           |         |
|     |     | text      | varchar(255) |           |         |
|     |     | parent_id | int          |     *     |         |
|     |     | likes     | int          |           |    0    |
