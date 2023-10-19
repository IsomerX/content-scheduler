FROM rust:buster as builder
ENV APP backend
# update rust (debian?)
RUN rustup update

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
RUN cargo build --release

FROM debian:buster
ENV APP backend
RUN apt update                      \
    && apt install -y libssl1.1     \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /target/release/$APP /usr/local/bin/$APP

EXPOSE 3123
ENTRYPOINT ["/usr/local/bin/backend"]