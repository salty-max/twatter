# Twatter

A Twitter clone in Rust

## Features

- [x] Create a post
  - [x] Limit the characters per post
  - [x] Optionally can be response to another post
  - [x] Optionally can be response to response
  - [x] Posts are stored in postgres
  - [x] Posts are validated before storage in database
- [x] Get a list of all top-level posts
  - [x] Text
  - [x] Likes
  - [x] Count of immediate children
- [ ] Get one post
  - [ ] Get immediate responses to the post
  - [ ] Text
  - [ ] Likes
- [ ] Update post
  - [ ] Text
  - [ ] Undelete
- [ ] Delete post
  - [ ] Soft delete post

## Tech

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
