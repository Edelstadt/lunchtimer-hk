FROM rust:latest

RUN rustup update nightly;
RUN rustup default nightly;

RUN apt-get update
RUN apt-get install libssl-dev pkg-config -y

WORKDIR /usr/src/lunchtimer-hk

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

#RUN cargo build --release

#RUN chmod +x /usr/src/lunchtimer-hk/target/release/lunchtime

CMD "tail -f /dev/null"
