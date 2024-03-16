FROM rust:latest

WORKDIR /usr/src/http_rs

COPY . .

RUN cargo build

ENV HTTP_RS_ADDR=0.0.0.0
ENV HTTP_RS_PORT=7878

EXPOSE $HTTP_RS_PORT

CMD cargo run
