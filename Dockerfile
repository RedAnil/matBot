ARG RUST_VERSION=1.96
ARG APP_NAME=matBot

FROM rust:${RUST_VERSION}-alpine3.22 AS build
ARG APP_NAME
WORKDIR /app

RUN apk add --no-cache musl-dev gcc sqlite-dev

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --locked
RUN rm -rf src

COPY src ./src
RUN touch src/main.rs
RUN cargo build --release --locked
RUN cp ./target/release/${APP_NAME} /bin/server

FROM alpine:3.22 AS final
RUN apk add --no-cache ca-certificates
COPY --from=build /bin/server /bin/server
ENTRYPOINT ["/bin/server"]