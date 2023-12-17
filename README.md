# Twatter

A Twitter clone in Rust

## Features

- [ ] Create a post
  - [ ] Limit the characters per post
  - [ ] Optionally can be response to another post
  - [ ] Optionally can be response to response
  - [ ] Posts are stored in postgres
  - [ ] Posts are validated before storage in database
- [ ] Get a list of all top-level posts
  - [ ] Text
  - [ ] Likes
  - [ ] Pagination
- [ ] Get one post
  - [ ] Get immediate responses to the post
    - [ ] Paginate
  - [ ] Text
  - [ ] Likes
- [ ] Update post
  - [ ] Text
  - [ ] Undelete
- [ ] Delete post
  - [ ] Soft delete post

## Tech

- Axum 0.7.2 
