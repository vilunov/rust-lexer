FROM rust:1.28-slim-jessie
WORKDIR /build
COPY . .
RUN cargo build --release
RUN strip target/release/rust-lexer

FROM debian:jessie-slim
COPY --from=0 /build/target/release/rust-lexer /bin/
WORKDIR /app
VOLUME /app

CMD rust-lexer

