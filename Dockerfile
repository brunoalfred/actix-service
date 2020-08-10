FROM rust:1.43.1

WORKDIR /usr/src/actix-service
COPY . .

RUN cargo install --path .

CMD ["actix-service"]