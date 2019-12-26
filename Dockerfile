FROM rust:1.40.0-alpine3.10

WORKDIR /usr/src/app

COPY . .

RUN cargo install --path .

CMD ["app"]