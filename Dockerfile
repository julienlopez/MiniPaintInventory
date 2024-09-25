FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install dioxus-cli

EXPOSE 8080

CMD [ "dx", "serve", "--platform", "fullstack", "--hot-reload" ]
