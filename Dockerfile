FROM rust:latest

RUN USER=root cargo new --bin rust-docker-web
WORKDIR /actix-service
COPY . .

RUN cargo install --path .

# CMD ["actix-service"]

CMD ["actix"]
