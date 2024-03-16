# Simple HTTP server

I just follow [this official rust guide](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)

## Single threaded

You can see the single threaded version on [this commit](https://github.com/Melvi-l/http_rs/commit/09eb0e916a97923628aa0746e6cf8b51fffd8f9b).

## Multi threaded

You can see the multi threaded version on all the more recent commit.

## Run the project

### Run with cargo

`cargo run`

### Run with docker

```sh
docker build . -t http_rs
docker run --rm -p 7878:7878 http_rs
```

ou

```sh
docker compose
```

