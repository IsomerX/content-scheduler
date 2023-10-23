FROM rust:1.68.1 as builder
WORKDIR /usr/local/src
COPY Cargo.toml Cargo.lock ./
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs
RUN cargo build --release 


COPY . .
RUN cargo build --release
RUN rm -rf target/release/*.*
RUN find target/release -mindepth 1 -maxdepth 1 -type d -print0 | xargs -0 rm -rf 
RUN mv target/release/* ./app

FROM ubuntu:20.04
RUN apt update                      \
    && apt install -y libssl1.1   \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/src/app /usr/local/bin/app
WORKDIR /usr/local/bin
CMD [ "./app" ]
