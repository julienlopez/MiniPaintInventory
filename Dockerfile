FROM rust:latest

WORKDIR /app

RUN cargo install cargo-binstall
RUN cargo binstall diesel_cli
RUN cargo binstall dioxus-cli
RUN rustup target add wasm32-unknown-unknown

EXPOSE 8080

COPY . .

CMD [ "dx", "serve", "--platform", "web", "--hot-reload", "true" ]
